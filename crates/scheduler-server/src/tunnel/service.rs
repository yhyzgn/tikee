//! gRPC Worker Tunnel service.

use scheduler_core::InstanceStatus;
use scheduler_proto::worker::v1::{
    Heartbeat, Ping, ServerMessage, TaskLog, TaskResult, WorkerMessage, WorkerRegistered,
    server_message, worker_message, worker_tunnel_service_server::WorkerTunnelService,
};
use scheduler_storage::{
    AppendJobInstanceLog, JobInstanceAttemptRepository, JobInstanceLogRepository,
    JobInstanceRepository, WorkflowRepository,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};

use super::WorkerRegistry;

const DEFAULT_LEASE_SECONDS: u64 = 30;

/// Worker Tunnel gRPC service implementation.
#[derive(Debug, Clone)]
pub struct WorkerTunnel {
    registry: WorkerRegistry,
    instances: JobInstanceRepository,
    logs: JobInstanceLogRepository,
    attempts: JobInstanceAttemptRepository,
    workflows: WorkflowRepository,
}

impl WorkerTunnel {
    /// Create a Worker Tunnel service backed by an in-memory registry.
    #[must_use]
    pub const fn new(
        registry: WorkerRegistry,
        instances: JobInstanceRepository,
        logs: JobInstanceLogRepository,
        attempts: JobInstanceAttemptRepository,
        workflows: WorkflowRepository,
    ) -> Self {
        Self {
            registry,
            instances,
            logs,
            attempts,
            workflows,
        }
    }
}

#[tonic::async_trait]
impl WorkerTunnelService for WorkerTunnel {
    type OpenTunnelStream = ReceiverStream<Result<ServerMessage, Status>>;

    async fn open_tunnel(
        &self,
        request: Request<Streaming<WorkerMessage>>,
    ) -> Result<Response<Self::OpenTunnelStream>, Status> {
        let mut inbound = request.into_inner();
        let registry = self.registry.clone();
        let instances = self.instances.clone();
        let logs = self.logs.clone();
        let attempts = self.attempts.clone();
        let workflows = self.workflows.clone();
        let (tx, rx) = mpsc::channel(16);
        let outbound = tx.clone();

        tokio::spawn(async move {
            while let Some(message) = inbound.message().await.transpose() {
                match message {
                    Ok(message) => {
                        let context = WorkerMessageContext {
                            registry: &registry,
                            instances: &instances,
                            logs: &logs,
                            attempts: &attempts,
                            workflows: &workflows,
                            tx: &tx,
                            outbound: &outbound,
                        };
                        if handle_worker_message(&context, message).await.is_err() {
                            break;
                        }
                    }
                    Err(status) => {
                        let _ = tx.send(Err(status)).await;
                        break;
                    }
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

struct WorkerMessageContext<'a> {
    registry: &'a WorkerRegistry,
    instances: &'a JobInstanceRepository,
    logs: &'a JobInstanceLogRepository,
    attempts: &'a JobInstanceAttemptRepository,
    workflows: &'a WorkflowRepository,
    tx: &'a mpsc::Sender<Result<ServerMessage, Status>>,
    outbound: &'a mpsc::Sender<Result<ServerMessage, Status>>,
}

async fn handle_worker_message(
    context: &WorkerMessageContext<'_>,
    message: WorkerMessage,
) -> Result<(), mpsc::error::SendError<Result<ServerMessage, Status>>> {
    match message.kind {
        Some(worker_message::Kind::Register(register)) => {
            let worker = context
                .registry
                .register(register, context.outbound.clone())
                .await;
            context
                .tx
                .send(Ok(ServerMessage {
                    kind: Some(server_message::Kind::Registered(WorkerRegistered {
                        worker_id: worker.worker_id,
                        lease_seconds: DEFAULT_LEASE_SECONDS,
                    })),
                }))
                .await
        }
        Some(worker_message::Kind::Heartbeat(Heartbeat {
            worker_id,
            sequence,
        })) => {
            let _ = context.registry.heartbeat(&worker_id, sequence).await;
            context
                .tx
                .send(Ok(ServerMessage {
                    kind: Some(server_message::Kind::Ping(Ping { sequence })),
                }))
                .await
        }
        Some(worker_message::Kind::TaskResult(result)) => {
            handle_task_result(context, result).await;
            Ok(())
        }
        Some(worker_message::Kind::TaskLog(TaskLog {
            worker_id,
            instance_id,
            level,
            message,
            sequence,
        })) => {
            if let Err(error) = context
                .logs
                .append(AppendJobInstanceLog {
                    instance_id,
                    worker_id,
                    level,
                    message,
                    sequence,
                })
                .await
            {
                tracing::warn!(%error, "failed to persist task log");
            }
            Ok(())
        }
        None => {
            context
                .tx
                .send(Err(Status::invalid_argument(
                    "worker message kind is required",
                )))
                .await
        }
    }
}

async fn handle_task_result(context: &WorkerMessageContext<'_>, result: TaskResult) {
    let TaskResult {
        worker_id,
        instance_id,
        success,
        ..
    } = result;
    let status = if success {
        InstanceStatus::Succeeded
    } else {
        InstanceStatus::Failed
    };
    match context
        .attempts
        .update_status(&instance_id, &worker_id, status)
        .await
    {
        Ok(Some(_)) => {
            if let Err(error) =
                refresh_broadcast_parent(context.instances, context.attempts, &instance_id).await
            {
                tracing::warn!(%error, %instance_id, "failed to refresh broadcast parent status");
            }
        }
        Ok(None) => {
            handle_single_task_result(context, &worker_id, &instance_id, success, status).await;
        }
        Err(error) => {
            tracing::warn!(%error, %instance_id, %worker_id, "failed to persist attempt result");
        }
    }
}

async fn handle_single_task_result(
    context: &WorkerMessageContext<'_>,
    worker_id: &str,
    instance_id: &str,
    success: bool,
    status: InstanceStatus,
) {
    if let Err(error) = context.instances.update_status(instance_id, status).await {
        tracing::warn!(%error, %instance_id, "failed to persist task result");
    }
    match context
        .workflows
        .complete_job_node_from_result(
            instance_id,
            status,
            Some(format!(
                "worker {worker_id} reported task success={success}"
            )),
        )
        .await
    {
        Ok(Some(outcome)) => {
            tracing::info!(
                workflow_instance_id = %outcome.workflow_instance_id,
                node_key = %outcome.node_key,
                status = %outcome.status,
                queued_nodes = ?outcome.queued_nodes,
                completed = outcome.completed,
                "workflow node advanced from worker task result"
            );
        }
        Ok(None) => {}
        Err(error) => {
            tracing::warn!(%error, %instance_id, "failed to advance workflow from task result");
        }
    }
}

async fn refresh_broadcast_parent(
    instances: &JobInstanceRepository,
    attempts: &JobInstanceAttemptRepository,
    instance_id: &str,
) -> Result<(), scheduler_storage::DbErr> {
    let children = attempts.list_by_instance(instance_id).await?;
    if children.is_empty() {
        return Ok(());
    }
    let all_done = children.iter().all(|attempt| {
        matches!(
            attempt.status,
            InstanceStatus::Succeeded | InstanceStatus::Failed
        )
    });
    if !all_done {
        return Ok(());
    }
    let status = if children
        .iter()
        .all(|attempt| attempt.status == InstanceStatus::Succeeded)
    {
        InstanceStatus::Succeeded
    } else {
        InstanceStatus::PartialFailed
    };
    let _ = instances.update_status(instance_id, status).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use scheduler_proto::worker::v1::{
        RegisterWorker, WorkerMessage, server_message, worker_message,
    };
    use scheduler_storage::{
        JobInstanceAttemptRepository, JobInstanceLogRepository, JobInstanceRepository,
        WorkflowRepository, connect_and_migrate,
    };
    use tokio::sync::mpsc;

    use super::{WorkerMessageContext, WorkerRegistry, handle_worker_message};

    #[tokio::test]
    async fn register_message_updates_registry_and_acknowledges_worker() {
        let registry = WorkerRegistry::default();
        let instances = instances().await;
        let logs = logs().await;
        let (tx, mut rx) = mpsc::channel(1);

        let attempts = attempts().await;

        let workflows = workflows().await;
        let context = WorkerMessageContext {
            registry: &registry,
            instances: &instances,
            logs: &logs,
            attempts: &attempts,
            workflows: &workflows,
            tx: &tx,
            outbound: &tx,
        };

        handle_worker_message(
            &context,
            WorkerMessage {
                kind: Some(worker_message::Kind::Register(RegisterWorker {
                    client_instance_id: "worker-1".to_owned(),
                    app: "billing".to_owned(),
                    namespace: "finance".to_owned(),
                    cluster: "prod".to_owned(),
                    region: "cn".to_owned(),
                    capabilities: Vec::new(),
                    labels: std::collections::HashMap::default(),
                })),
            },
        )
        .await
        .unwrap_or_else(|error| panic!("ack should send: {error}"));

        let ack = rx
            .recv()
            .await
            .unwrap_or_else(|| panic!("ack should exist"))
            .unwrap_or_else(|error| panic!("ack should be ok: {error}"));

        match ack.kind {
            Some(server_message::Kind::Registered(registered)) => {
                assert!(registered.worker_id.starts_with("wrk-"));
            }
            other => panic!("unexpected ack: {other:?}"),
        }

        let registered_id = registry
            .worker_ids()
            .await
            .into_iter()
            .next()
            .unwrap_or_else(|| panic!("registered worker id should exist"));
        assert!(registry.get(&registered_id).await.is_some());
    }

    async fn instances() -> JobInstanceRepository {
        let db = connect_and_migrate("sqlite::memory:")
            .await
            .unwrap_or_else(|error| panic!("test storage should initialize: {error}"));
        JobInstanceRepository::new(db)
    }

    async fn attempts() -> JobInstanceAttemptRepository {
        let db = connect_and_migrate("sqlite::memory:")
            .await
            .unwrap_or_else(|error| panic!("test storage should initialize: {error}"));
        JobInstanceAttemptRepository::new(db)
    }

    async fn workflows() -> WorkflowRepository {
        let db = connect_and_migrate("sqlite::memory:")
            .await
            .unwrap_or_else(|error| panic!("test storage should initialize: {error}"));
        WorkflowRepository::new(db)
    }

    async fn logs() -> JobInstanceLogRepository {
        let db = connect_and_migrate("sqlite::memory:")
            .await
            .unwrap_or_else(|error| panic!("test storage should initialize: {error}"));
        JobInstanceLogRepository::new(db)
    }
}
