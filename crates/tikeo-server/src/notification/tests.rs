use super::*;
use tikeo_core::{ExecutionMode, TriggerType};
use tikeo_storage::{
    CreateJob, CreateJobInstance, CreateNotificationChannel, CreateNotificationPolicy,
    CreateNotificationTemplate, JobInstanceRepository, NotificationDeliveryAttemptFilters,
    NotificationMessageFilters, NotificationTemplateRepository, connect_and_migrate,
};

#[tokio::test]
async fn due_delivery_attempts_post_to_webhook_and_update_message_status() {
    let db = connect_and_migrate("sqlite::memory:")
        .await
        .unwrap_or_else(|error| panic!("test storage should initialize: {error}"));
    let jobs = JobRepository::new(db.clone());
    let instances = JobInstanceRepository::new(db.clone());
    let channels = NotificationChannelRepository::new(db.clone());
    let policies = NotificationPolicyRepository::new(db.clone());
    let messages = NotificationMessageRepository::new(db.clone());
    let attempts = NotificationDeliveryAttemptRepository::new(db.clone());
    let received = std::sync::Arc::new(tokio::sync::Mutex::new(None::<serde_json::Value>));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap_or_else(|error| panic!("webhook listener should bind: {error}"));
    let url = format!(
        "http://{}/notify/top-secret-token",
        listener
            .local_addr()
            .unwrap_or_else(|error| panic!("listener addr should read: {error}"))
    );
    let received_for_route = received.clone();
    let app = axum::Router::new().route(
        "/notify/top-secret-token",
        axum::routing::post(move |axum::Json(payload): axum::Json<serde_json::Value>| {
            let received = received_for_route.clone();
            async move {
                *received.lock().await = Some(payload);
                axum::http::StatusCode::OK
            }
        }),
    );
    let server = tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .unwrap_or_else(|error| panic!("webhook server should run: {error}"));
    });

    let job = jobs
        .create_job(CreateJob {
            created_by: None,
            namespace: "default".to_owned(),
            app: "billing".to_owned(),
            name: "billing-nightly".to_owned(),
            schedule_type: "api".to_owned(),
            schedule_expr: None,
            misfire_policy: "fire_once".to_owned(),
            schedule_start_at: None,
            schedule_end_at: None,
            schedule_calendar_json: None,
            processor_name: Some("demo.fail".to_owned()),
            processor_type: None,
            script_id: None,
            enabled: true,
            canary_job_id: None,
            canary_percent: 0,
            retry_policy: None,
        })
        .await
        .unwrap_or_else(|error| panic!("job should create: {error}"));
    let instance = instances
        .create_pending(CreateJobInstance {
            job_id: job.id.clone(),
            trigger_type: TriggerType::Api,
            execution_mode: ExecutionMode::Single,
        })
        .await
        .unwrap_or_else(|error| panic!("instance should create: {error}"))
        .unwrap_or_else(|| panic!("instance should exist"));
    let instance = instances
        .update_status(&instance.id, InstanceStatus::Failed)
        .await
        .unwrap_or_else(|error| panic!("status should update: {error}"))
        .unwrap_or_else(|| panic!("instance should exist"));
    let channel = channels
        .create_channel(CreateNotificationChannel {
            scope_type: "app".to_owned(),
            namespace: Some("default".to_owned()),
            app: Some("billing".to_owned()),
            worker_pool: None,
            name: "ops".to_owned(),
            provider: "webhook".to_owned(),
            enabled: true,
            config_json: serde_json::json!({
                "url": url,
                "messageType": "json",
                "template": {
                    "body": {
                        "eventType": "{{eventType}}",
                        "subject": "{{subject}}",
                        "summary": "Channel template rendered {{body}}"
                    }
                }
            })
            .to_string(),
            secret_refs_json: "{}".to_owned(),
            safety_policy_json: Some(
                serde_json::json!({"allowInsecureLoopback": true}).to_string(),
            ),
        })
        .await
        .unwrap_or_else(|error| panic!("channel should create: {error}"));
    policies
        .create_policy(CreateNotificationPolicy {
            owner_type: "job".to_owned(),
            owner_id: Some(job.id.clone()),
            name: "job failures".to_owned(),
            event_family: "job_instance".to_owned(),
            event_filter_json: serde_json::json!({"statuses":["failed"]}).to_string(),
            channel_refs_json: serde_json::json!([{"channelId": channel.id}]).to_string(),
            template_ref: None,
            severity: "critical".to_owned(),
            enabled: true,
            dedupe_seconds: 300,
        })
        .await
        .unwrap_or_else(|error| panic!("policy should create: {error}"));

    let center = NotificationCenter::new(
        channels.clone(),
        policies,
        messages.clone(),
        attempts.clone(),
        tikeo_storage::NotificationTemplateRepository::new(channels.db()),
        jobs,
    );
    center
        .emit_job_instance_event(&instance, JobNotificationEvent::Failed, Some("exit 2"))
        .await
        .unwrap_or_else(|error| panic!("notification should emit: {error}"));
    let delivered = process_due_notification_delivery_attempts(
        &channels,
        &messages,
        &attempts,
        50,
        NotificationDeliveryPolicy {
            max_attempts: 3,
            backoff_seconds: 300,
        },
    )
    .await
    .unwrap_or_else(|error| panic!("delivery processor should run: {error}"));

    assert_eq!(delivered.scanned, 1);
    assert_eq!(delivered.delivered, 1);
    let stored_payload = received
        .lock()
        .await
        .clone()
        .unwrap_or_else(|| panic!("webhook should receive payload"));
    assert_eq!(stored_payload["eventType"], "job_instance.failed");
    assert_eq!(
        stored_payload["subject"],
        "Tikeo job billing-nightly: failed"
    );
    assert!(
        stored_payload["summary"]
            .as_str()
            .unwrap_or_default()
            .starts_with("Channel template rendered Job billing-nightly instance "),
        "channel-level webhook template should render when no policy template is linked: {stored_payload}"
    );
    assert!(!stored_payload.to_string().contains("top-secret-token"));
    let timeline = messages
        .list_messages(NotificationMessageFilters {
            source_type: Some("job_instance".to_owned()),
            source_id: Some(instance.id.clone()),
            ..Default::default()
        })
        .await
        .unwrap_or_else(|error| panic!("messages should list: {error}"));
    assert_eq!(timeline[0].status, "delivered");
    let attempts_list = attempts
        .list_attempts(NotificationDeliveryAttemptFilters::default())
        .await
        .unwrap_or_else(|error| panic!("attempts should list: {error}"));
    assert!(attempts_list.iter().any(|attempt| attempt.attempt == 0
        && !attempt.delivered
        && attempt.retry_state == "retry_consumed"));
    assert!(attempts_list.iter().any(|attempt| attempt.attempt == 1
        && attempt.delivered
        && attempt.retry_state == "delivered"));
    assert!(attempts_list.iter().any(|attempt| attempt.delivered
        && attempt.retry_state == "delivered"
        && !attempt.target_redacted.contains("top-secret-token")));
    server.abort();
}

#[tokio::test]
async fn webhook_delivery_injects_authorization_from_secret_refs_without_leaking_it() {
    let db = connect_and_migrate("sqlite::memory:")
        .await
        .unwrap_or_else(|error| panic!("test storage should initialize: {error}"));
    let jobs = JobRepository::new(db.clone());
    let instances = JobInstanceRepository::new(db.clone());
    let channels = NotificationChannelRepository::new(db.clone());
    let policies = NotificationPolicyRepository::new(db.clone());
    let messages = NotificationMessageRepository::new(db.clone());
    let attempts = NotificationDeliveryAttemptRepository::new(db.clone());
    let received_headers =
        std::sync::Arc::new(tokio::sync::Mutex::new((None::<String>, None::<String>)));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap_or_else(|error| panic!("webhook listener should bind: {error}"));
    let url = format!(
        "http://{}/notify",
        listener
            .local_addr()
            .unwrap_or_else(|error| panic!("listener addr should read: {error}"))
    );
    let received_for_route = received_headers.clone();
    let app = axum::Router::new().route(
        "/notify",
        axum::routing::post(move |headers: axum::http::HeaderMap| {
            let received = received_for_route.clone();
            async move {
                *received.lock().await = (
                    headers
                        .get(axum::http::header::AUTHORIZATION)
                        .and_then(|value| value.to_str().ok())
                        .map(ToOwned::to_owned),
                    headers
                        .get("x-tikeo-secret-header")
                        .and_then(|value| value.to_str().ok())
                        .map(ToOwned::to_owned),
                );
                axum::http::StatusCode::OK
            }
        }),
    );
    let server = tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .unwrap_or_else(|error| panic!("webhook server should run: {error}"));
    });

    let job = jobs
        .create_job(CreateJob {
            created_by: None,
            namespace: "default".to_owned(),
            app: "billing".to_owned(),
            name: "billing-nightly".to_owned(),
            schedule_type: "api".to_owned(),
            schedule_expr: None,
            misfire_policy: "fire_once".to_owned(),
            schedule_start_at: None,
            schedule_end_at: None,
            schedule_calendar_json: None,
            processor_name: Some("demo.fail".to_owned()),
            processor_type: None,
            script_id: None,
            enabled: true,
            canary_job_id: None,
            canary_percent: 0,
            retry_policy: None,
        })
        .await
        .unwrap_or_else(|error| panic!("job should create: {error}"));
    let instance = instances
        .create_pending(CreateJobInstance {
            job_id: job.id.clone(),
            trigger_type: TriggerType::Api,
            execution_mode: ExecutionMode::Single,
        })
        .await
        .unwrap_or_else(|error| panic!("instance should create: {error}"))
        .unwrap_or_else(|| panic!("instance should exist"));
    let instance = instances
        .update_status(&instance.id, InstanceStatus::Failed)
        .await
        .unwrap_or_else(|error| panic!("status should update: {error}"))
        .unwrap_or_else(|| panic!("instance should exist"));
    let expected_authorization = std::env::var("PATH")
        .unwrap_or_else(|error| panic!("PATH should be available for secret-ref test: {error}"));
    let channel = channels
        .create_channel(CreateNotificationChannel {
            scope_type: "app".to_owned(),
            namespace: Some("default".to_owned()),
            app: Some("billing".to_owned()),
            worker_pool: None,
            name: "ops".to_owned(),
            provider: "webhook".to_owned(),
            enabled: true,
            config_json: serde_json::json!({"url": url}).to_string(),
            secret_refs_json: serde_json::json!({
                "authorization": "env:PATH",
                "headers": {"x-tikeo-secret-header": "env:PATH"}
            })
            .to_string(),
            safety_policy_json: Some(
                serde_json::json!({"allowInsecureLoopback": true}).to_string(),
            ),
        })
        .await
        .unwrap_or_else(|error| panic!("channel should create: {error}"));
    policies
        .create_policy(CreateNotificationPolicy {
            owner_type: "job".to_owned(),
            owner_id: Some(job.id.clone()),
            name: "job failures".to_owned(),
            event_family: "job_instance".to_owned(),
            event_filter_json: serde_json::json!({"statuses":["failed"]}).to_string(),
            channel_refs_json: serde_json::json!([{"channelId": channel.id}]).to_string(),
            template_ref: None,
            severity: "critical".to_owned(),
            enabled: true,
            dedupe_seconds: 300,
        })
        .await
        .unwrap_or_else(|error| panic!("policy should create: {error}"));

    let center = NotificationCenter::new(
        channels.clone(),
        policies,
        messages.clone(),
        attempts.clone(),
        tikeo_storage::NotificationTemplateRepository::new(channels.db()),
        jobs,
    );
    center
        .emit_job_instance_event(&instance, JobNotificationEvent::Failed, Some("exit 2"))
        .await
        .unwrap_or_else(|error| panic!("notification should emit: {error}"));
    let delivered = process_due_notification_delivery_attempts(
        &channels,
        &messages,
        &attempts,
        50,
        NotificationDeliveryPolicy::default(),
    )
    .await
    .unwrap_or_else(|error| panic!("delivery processor should run: {error}"));

    assert_eq!(delivered.delivered, 1);
    let received_headers = received_headers.lock().await.clone();
    assert_eq!(
        received_headers.0.as_deref(),
        Some(expected_authorization.as_str())
    );
    assert_eq!(
        received_headers.1.as_deref(),
        Some(expected_authorization.as_str())
    );
    let attempts_list = attempts
        .list_attempts(NotificationDeliveryAttemptFilters::default())
        .await
        .unwrap_or_else(|error| panic!("attempts should list: {error}"));
    assert!(!attempts_list.iter().any(|attempt| {
        attempt
            .error
            .as_deref()
            .unwrap_or_default()
            .contains(&expected_authorization)
    }));
    server.abort();
}

#[test]
fn email_channel_accepts_metadata_secret_refs_password_alias() {
    let channel = NotificationChannelDeliveryConfig {
        id: "notification-channel-email".to_owned(),
        provider: "email".to_owned(),
        enabled: true,
        config_json: serde_json::json!({
            "to": ["ops@example.com"],
            "smtpUrl": "smtp+starttls://smtp.example.com:587",
            "username": "tikeo"
        })
        .to_string(),
        secret_refs_json: serde_json::json!({
            "password": "env:TIKEO_SMTP_PASSWORD"
        })
        .to_string(),
        target_redacted: "email".to_owned(),
        safety_policy_json: None,
    };

    let resolved = notification_channel_from_delivery_config(&channel)
        .unwrap_or_else(|| panic!("email channel should resolve from metadata-shaped secretRefs"));
    match resolved {
        NotificationChannel::Email {
            password_secret_ref,
            ..
        } => {
            assert_eq!(
                password_secret_ref.as_deref(),
                Some("env:TIKEO_SMTP_PASSWORD")
            );
        }
        other => panic!("expected email channel, got {other:?}"),
    }
}

#[tokio::test]
async fn job_instance_event_materializes_message_and_delivery_attempts() {
    let db = connect_and_migrate("sqlite::memory:")
        .await
        .unwrap_or_else(|error| panic!("test storage should initialize: {error}"));
    let jobs = JobRepository::new(db.clone());
    let instances = JobInstanceRepository::new(db.clone());
    let channels = NotificationChannelRepository::new(db.clone());
    let policies = NotificationPolicyRepository::new(db.clone());
    let messages = NotificationMessageRepository::new(db.clone());
    let attempts = NotificationDeliveryAttemptRepository::new(db.clone());
    let job = jobs
        .create_job(CreateJob {
            created_by: None,
            namespace: "default".to_owned(),
            app: "billing".to_owned(),
            name: "billing-nightly".to_owned(),
            schedule_type: "api".to_owned(),
            schedule_expr: None,
            misfire_policy: "fire_once".to_owned(),
            schedule_start_at: None,
            schedule_end_at: None,
            schedule_calendar_json: None,
            processor_name: Some("demo.fail".to_owned()),
            processor_type: None,
            script_id: None,
            enabled: true,
            canary_job_id: None,
            canary_percent: 0,
            retry_policy: None,
        })
        .await
        .unwrap_or_else(|error| panic!("job should create: {error}"));
    let instance = instances
        .create_pending(CreateJobInstance {
            job_id: job.id.clone(),
            trigger_type: TriggerType::Api,
            execution_mode: ExecutionMode::Single,
        })
        .await
        .unwrap_or_else(|error| panic!("instance should create: {error}"))
        .unwrap_or_else(|| panic!("instance should exist"));
    let instance = instances
        .update_status(&instance.id, InstanceStatus::Failed)
        .await
        .unwrap_or_else(|error| panic!("status should update: {error}"))
        .unwrap_or_else(|| panic!("instance should exist"));
    let channel = channels
        .create_channel(CreateNotificationChannel {
            scope_type: "app".to_owned(),
            namespace: Some("default".to_owned()),
            app: Some("billing".to_owned()),
            worker_pool: None,
            name: "ops".to_owned(),
            provider: "webhook".to_owned(),
            enabled: true,
            config_json:
                serde_json::json!({"url":"https://hooks.example.com/services/top-secret-token"})
                    .to_string(),
            secret_refs_json: "{}".to_owned(),
            safety_policy_json: None,
        })
        .await
        .unwrap_or_else(|error| panic!("channel should create: {error}"));
    policies
        .create_policy(CreateNotificationPolicy {
            owner_type: "job".to_owned(),
            owner_id: Some(job.id.clone()),
            name: "job failures".to_owned(),
            event_family: "job_instance".to_owned(),
            event_filter_json: serde_json::json!({"statuses":["failed"]}).to_string(),
            channel_refs_json: serde_json::json!([{"channelId": channel.id}]).to_string(),
            template_ref: None,
            severity: "critical".to_owned(),
            enabled: true,
            dedupe_seconds: 300,
        })
        .await
        .unwrap_or_else(|error| panic!("policy should create: {error}"));

    let center = NotificationCenter::new(
        channels.clone(),
        policies,
        messages.clone(),
        attempts.clone(),
        tikeo_storage::NotificationTemplateRepository::new(channels.db()),
        jobs,
    );
    let emitted = center
        .emit_job_instance_event(&instance, JobNotificationEvent::Failed, Some("exit 2"))
        .await
        .unwrap_or_else(|error| panic!("notification should emit: {error}"));

    assert_eq!(emitted.matched_policies, 1);
    assert_eq!(emitted.messages_created, 1);
    assert_eq!(emitted.delivery_attempts_created, 1);
    let deduped = center
        .emit_job_instance_event(&instance, JobNotificationEvent::Failed, Some("exit 2"))
        .await
        .unwrap_or_else(|error| panic!("duplicate notification should dedupe: {error}"));
    assert_eq!(deduped.matched_policies, 1);
    assert_eq!(deduped.messages_created, 0);
    assert_eq!(deduped.delivery_attempts_created, 0);
    let timeline = messages
        .list_messages(NotificationMessageFilters {
            source_type: Some("job_instance".to_owned()),
            source_id: Some(instance.id.clone()),
            ..Default::default()
        })
        .await
        .unwrap_or_else(|error| panic!("messages should list: {error}"));
    assert_eq!(timeline[0].event_type, "job_instance.failed");
    assert!(!timeline[0].payload_json.contains("top-secret-token"));
    let delivery = attempts
        .list_attempts(NotificationDeliveryAttemptFilters::default())
        .await
        .unwrap_or_else(|error| panic!("attempts should list: {error}"));
    assert_eq!(delivery[0].retry_state, "retry_pending");
    assert_eq!(delivery[0].target_redacted, "https://hooks.example.com/...");
}

#[tokio::test]
async fn policy_template_ref_materializes_and_drives_provider_payload() {
    let db = connect_and_migrate("sqlite::memory:")
        .await
        .unwrap_or_else(|error| panic!("test storage should initialize: {error}"));
    let jobs = JobRepository::new(db.clone());
    let instances = JobInstanceRepository::new(db.clone());
    let channels = NotificationChannelRepository::new(db.clone());
    let policies = NotificationPolicyRepository::new(db.clone());
    let messages = NotificationMessageRepository::new(db.clone());
    let attempts = NotificationDeliveryAttemptRepository::new(db.clone());
    let templates = NotificationTemplateRepository::new(db.clone());
    let received = std::sync::Arc::new(tokio::sync::Mutex::new(None::<serde_json::Value>));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap_or_else(|error| panic!("webhook listener should bind: {error}"));
    let url = format!(
        "http://{}/notify",
        listener
            .local_addr()
            .unwrap_or_else(|error| panic!("listener addr should read: {error}"))
    );
    let received_for_route = received.clone();
    let app = axum::Router::new().route(
        "/notify",
        axum::routing::post(move |axum::Json(payload): axum::Json<serde_json::Value>| {
            let received = received_for_route.clone();
            async move {
                *received.lock().await = Some(payload);
                axum::http::StatusCode::OK
            }
        }),
    );
    let server = tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .unwrap_or_else(|error| panic!("webhook server should run: {error}"));
    });

    let job = jobs
        .create_job(CreateJob {
            created_by: None,
            namespace: "default".to_owned(),
            app: "billing".to_owned(),
            name: "billing-nightly".to_owned(),
            schedule_type: "api".to_owned(),
            schedule_expr: None,
            misfire_policy: "fire_once".to_owned(),
            schedule_start_at: None,
            schedule_end_at: None,
            schedule_calendar_json: None,
            processor_name: Some("demo.fail".to_owned()),
            processor_type: None,
            script_id: None,
            enabled: true,
            canary_job_id: None,
            canary_percent: 0,
            retry_policy: None,
        })
        .await
        .unwrap_or_else(|error| panic!("job should create: {error}"));
    let instance = instances
        .create_pending(CreateJobInstance {
            job_id: job.id.clone(),
            trigger_type: TriggerType::Api,
            execution_mode: ExecutionMode::Single,
        })
        .await
        .unwrap_or_else(|error| panic!("instance should create: {error}"))
        .unwrap_or_else(|| panic!("instance should exist"));
    let instance = instances
        .update_status(&instance.id, InstanceStatus::Failed)
        .await
        .unwrap_or_else(|error| panic!("status should update: {error}"))
        .unwrap_or_else(|| panic!("instance should exist"));
    let template = templates
        .create_template(CreateNotificationTemplate {
            template_key: "ops.webhook.failure".to_owned(),
            name: "Ops webhook failure".to_owned(),
            description: Some("Provider-specific webhook template".to_owned()),
            provider: "webhook".to_owned(),
            message_type: "json".to_owned(),
            enabled: true,
            body_json: serde_json::json!({
                "subject": "Templated {{subject}}",
                "text": "Rendered {{body}} / {{eventType}}",
                "body": {
                    "summary": "{{subject}}",
                    "details": "{{body}}",
                    "event": "{{eventType}}",
                    "resource": "{{resourceId}}"
                }
            })
            .to_string(),
            variables_json: serde_json::json!({"severity":"critical"}).to_string(),
        })
        .await
        .unwrap_or_else(|error| panic!("template should create: {error}"));
    let channel = channels
        .create_channel(CreateNotificationChannel {
            scope_type: "app".to_owned(),
            namespace: Some("default".to_owned()),
            app: Some("billing".to_owned()),
            worker_pool: None,
            name: "ops".to_owned(),
            provider: "webhook".to_owned(),
            enabled: true,
            config_json: serde_json::json!({"url": url}).to_string(),
            secret_refs_json: "{}".to_owned(),
            safety_policy_json: Some(
                serde_json::json!({"allowInsecureLoopback": true}).to_string(),
            ),
        })
        .await
        .unwrap_or_else(|error| panic!("channel should create: {error}"));
    policies
        .create_policy(CreateNotificationPolicy {
            owner_type: "job".to_owned(),
            owner_id: Some(job.id.clone()),
            name: "job failures".to_owned(),
            event_family: "job_instance".to_owned(),
            event_filter_json: serde_json::json!({"statuses":["failed"]}).to_string(),
            channel_refs_json: serde_json::json!([{"channelId": channel.id}]).to_string(),
            template_ref: Some(template.template_key.clone()),
            severity: "critical".to_owned(),
            enabled: true,
            dedupe_seconds: 300,
        })
        .await
        .unwrap_or_else(|error| panic!("policy should create: {error}"));

    let center = NotificationCenter::new(
        channels.clone(),
        policies,
        messages.clone(),
        attempts.clone(),
        templates,
        jobs,
    );
    center
        .emit_job_instance_event(&instance, JobNotificationEvent::Failed, Some("exit 2"))
        .await
        .unwrap_or_else(|error| panic!("notification should emit: {error}"));

    let timeline = messages
        .list_messages(NotificationMessageFilters {
            source_type: Some("job_instance".to_owned()),
            source_id: Some(instance.id.clone()),
            ..Default::default()
        })
        .await
        .unwrap_or_else(|error| panic!("messages should list: {error}"));
    assert_eq!(timeline.len(), 1);
    assert_eq!(
        timeline[0].subject,
        "Templated Tikeo job billing-nightly: failed"
    );
    assert!(
        timeline[0]
            .body
            .starts_with("Rendered Job billing-nightly instance "),
        "body should be rendered from the reusable template: {}",
        timeline[0].body
    );
    assert!(timeline[0].body.ends_with(" / job_instance.failed"));
    let payload: serde_json::Value = serde_json::from_str(&timeline[0].payload_json)
        .unwrap_or_else(|error| panic!("payload should be JSON: {error}"));
    assert_eq!(payload["templateKey"], "ops.webhook.failure");
    assert_eq!(payload["template"]["body"]["event"], "job_instance.failed");

    let delivered = process_due_notification_delivery_attempts(
        &channels,
        &messages,
        &attempts,
        50,
        NotificationDeliveryPolicy::default(),
    )
    .await
    .unwrap_or_else(|error| panic!("delivery processor should run: {error}"));
    assert_eq!(delivered.delivered, 1);
    let stored_payload = received
        .lock()
        .await
        .clone()
        .unwrap_or_else(|| panic!("webhook should receive payload"));
    assert_eq!(stored_payload["summary"], timeline[0].subject);
    assert_eq!(stored_payload["event"], "job_instance.failed");
    assert_ne!(
        stored_payload["summary"],
        "INLINE CHANNEL TEMPLATE SHOULD NOT WIN"
    );
    assert_eq!(stored_payload["details"], timeline[0].body);
    assert_eq!(stored_payload["event"], "job_instance.failed");
    server.abort();
}

#[tokio::test]
async fn rich_provider_delivery_fails_closed_without_required_template() {
    let message = sample_notification_message();
    let client = NotificationProviderClient::new(AlertDeliveryPolicy {
        allow_insecure_loopback: true,
    });

    for (provider, config) in [
        (
            "dingtalk",
            serde_json::json!({"url": "http://127.0.0.1:9/notify", "messageType": "actionCard"}),
        ),
        (
            "feishu",
            serde_json::json!({"url": "http://127.0.0.1:9/notify", "messageType": "image"}),
        ),
        (
            "wechat_work",
            serde_json::json!({"url": "http://127.0.0.1:9/notify", "messageType": "template_card"}),
        ),
    ] {
        let result = client
            .deliver(
                &NotificationChannelDeliveryConfig {
                    id: format!("channel-{provider}"),
                    provider: provider.to_owned(),
                    enabled: true,
                    config_json: config.to_string(),
                    secret_refs_json: "{}".to_owned(),
                    target_redacted: "local".to_owned(),
                    safety_policy_json: Some(
                        serde_json::json!({"allowInsecureLoopback": true}).to_string(),
                    ),
                },
                &message,
            )
            .await;
        assert!(
            !result.delivered,
            "{provider} should fail closed without template"
        );
        assert!(
            result
                .error
                .as_deref()
                .is_some_and(|error| error.contains("requires a channel inline template")),
            "{provider} error should explain template requirement: {result:?}"
        );
    }
}

#[tokio::test]
async fn provider_delivery_renders_configured_message_types_and_templates() {
    let received = std::sync::Arc::new(tokio::sync::Mutex::new(Vec::<serde_json::Value>::new()));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap_or_else(|error| panic!("webhook listener should bind: {error}"));
    let url = format!(
        "http://{}/notify",
        listener
            .local_addr()
            .unwrap_or_else(|error| panic!("listener addr should read: {error}"))
    );
    let received_for_route = received.clone();
    let app = axum::Router::new().route(
        "/notify",
        axum::routing::post(move |axum::Json(payload): axum::Json<serde_json::Value>| {
            let received = received_for_route.clone();
            async move {
                received.lock().await.push(payload);
                axum::http::StatusCode::OK
            }
        }),
    );
    let server = tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .unwrap_or_else(|error| panic!("webhook server should run: {error}"));
    });

    let message = sample_notification_message();
    let client = NotificationProviderClient::new(AlertDeliveryPolicy {
        allow_insecure_loopback: true,
    });

    for (provider, config) in [
        (
            "slack",
            serde_json::json!({"url": url, "messageType": "blockKit", "template": {"text": "{{subject}}", "blocks": [{"type":"section","text":{"type":"mrkdwn","text":"{{body}} / {{severity}}"}}]}}),
        ),
        (
            "dingtalk",
            serde_json::json!({"url": url, "messageType": "markdown", "template": {"title": "{{subject}}", "text": "{{body}} {{eventType}}"}}),
        ),
        (
            "feishu",
            serde_json::json!({"url": url, "messageType": "post", "template": {"post": {"zh_cn": {"title": "{{subject}}", "content": [[{"tag":"text","text":"{{body}}"}]]}}}}),
        ),
        (
            "feishu",
            serde_json::json!({"url": url, "messageType": "image", "template": {"imageKey":"{{resourceId}}-image"}}),
        ),
        (
            "feishu",
            serde_json::json!({"url": url, "messageType": "share_chat", "template": {"shareChatId":"{{resourceId}}-chat"}}),
        ),
        (
            "wechat_work",
            serde_json::json!({"url": url, "messageType": "news", "template": {"articles": [{"title":"{{subject}}","description":"{{body}}","url":"https://example.com/{{resourceId}}"}]}}),
        ),
        (
            "wechat_work",
            serde_json::json!({"url": url, "messageType": "voice", "template": {"media_id":"{{resourceId}}-voice"}}),
        ),
        (
            "pagerduty",
            serde_json::json!({"url": url, "routingKey": "route-123", "messageType": "resolve", "template": {"summary": "{{subject}}", "dedupKey": "custom-{{messageId}}", "source": "{{resourceType}}", "severity": "error", "timestamp": "{{triggeredAt}}", "component": "{{resourceId}}", "client":"tikeo", "clientUrl":"https://example.com/{{resourceId}}", "links":[{"href":"https://example.com/{{resourceId}}","text":"runbook"}], "images":"[{\"src\":\"https://example.com/{{resourceId}}.png\",\"alt\":\"chart\"}]"}}),
        ),
        (
            "webhook",
            serde_json::json!({"url": url, "template": {"provider":"generic","message":"{{subject}}","dedupe":"{{dedupeKey}}"}}),
        ),
        (
            "ops_bridge",
            serde_json::json!({"url": url, "messageType": "ticket", "template": {"body": {"ticket":"{{subject}}","event":"{{eventType}}"}}}),
        ),
    ] {
        let result = client
            .deliver(
                &NotificationChannelDeliveryConfig {
                    id: format!("channel-{provider}"),
                    provider: provider.to_owned(),
                    enabled: true,
                    config_json: config.to_string(),
                    secret_refs_json: if provider == "pagerduty" {
                        serde_json::json!({"routingKey":"env:PATH"}).to_string()
                    } else {
                        "{}".to_owned()
                    },
                    target_redacted: "local".to_owned(),
                    safety_policy_json: Some(
                        serde_json::json!({"allowInsecureLoopback": true}).to_string(),
                    ),
                },
                &message,
            )
            .await;
        assert!(result.delivered, "{provider} should deliver: {result:?}");
    }

    let payloads = received.lock().await.clone();
    assert_eq!(payloads[0]["text"], "Job failed");
    assert_eq!(
        payloads[0]["blocks"][0]["text"]["text"],
        "Exited 2 / critical"
    );
    assert_eq!(payloads[1]["msgtype"], "markdown");
    assert_eq!(
        payloads[1]["markdown"]["text"],
        "Exited 2 job_instance.failed"
    );
    assert_eq!(payloads[2]["msg_type"], "post");
    assert_eq!(
        payloads[2]["content"]["post"]["zh_cn"]["title"],
        "Job failed"
    );
    assert_eq!(payloads[3]["msg_type"], "image");
    assert_eq!(payloads[3]["content"]["image_key"], "billing-nightly-image");
    assert_eq!(payloads[4]["msg_type"], "share_chat");
    assert_eq!(
        payloads[4]["content"]["share_chat_id"],
        "billing-nightly-chat"
    );
    assert_eq!(payloads[5]["msgtype"], "news");
    assert_eq!(
        payloads[5]["news"]["articles"][0]["description"],
        "Exited 2"
    );
    assert_eq!(payloads[6]["msgtype"], "voice");
    assert_eq!(payloads[6]["voice"]["media_id"], "billing-nightly-voice");
    assert_eq!(payloads[7]["event_action"], "resolve");
    assert_eq!(payloads[7]["dedup_key"], "custom-msg-1");
    assert_eq!(payloads[7]["payload"]["source"], "job");
    assert_eq!(payloads[7]["payload"]["severity"], "error");
    assert_eq!(payloads[7]["payload"]["timestamp"], "2026-06-11T00:00:00Z");
    assert_eq!(payloads[7]["client"], "tikeo");
    assert_eq!(
        payloads[7]["client_url"],
        "https://example.com/billing-nightly"
    );
    assert_eq!(payloads[7]["links"][0]["text"], "runbook");
    assert_eq!(payloads[7]["images"][0]["alt"], "chart");
    assert_eq!(payloads[8]["message"], "Job failed");
    assert_eq!(payloads[8]["dedupe"], "policy-1:instance-1:failed");
    assert_eq!(payloads[9]["ticket"], "Job failed");
    assert_eq!(payloads[9]["event"], "job_instance.failed");
    server.abort();
}

#[tokio::test]
async fn provider_delivery_covers_all_builtin_message_shape_families() {
    let received = std::sync::Arc::new(tokio::sync::Mutex::new(Vec::<serde_json::Value>::new()));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap_or_else(|error| panic!("webhook listener should bind: {error}"));
    let url = format!(
        "http://{}/notify",
        listener
            .local_addr()
            .unwrap_or_else(|error| panic!("listener addr should read: {error}"))
    );
    let received_for_route = received.clone();
    let app = axum::Router::new().route(
        "/notify",
        axum::routing::post(move |axum::Json(payload): axum::Json<serde_json::Value>| {
            let received = received_for_route.clone();
            async move {
                received.lock().await.push(payload);
                axum::http::StatusCode::OK
            }
        }),
    );
    let server = tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .unwrap_or_else(|error| panic!("webhook server should run: {error}"));
    });

    let message = sample_notification_message();
    let client = NotificationProviderClient::new(AlertDeliveryPolicy {
        allow_insecure_loopback: true,
    });
    let cases = [
        (
            "slack",
            serde_json::json!({"url": url, "messageType": "attachments", "template": {"text": "{{subject}}", "attachments": [{"title":"{{subject}}","text":"{{body}}"}]}}),
        ),
        (
            "dingtalk",
            serde_json::json!({"url": url, "messageType": "text", "template": {"content": "{{subject}} {{body}}"}}),
        ),
        (
            "dingtalk",
            serde_json::json!({"url": url, "messageType": "link", "template": {"title": "{{subject}}", "text": "{{body}}", "messageUrl": "https://example.com/{{resourceId}}"}}),
        ),
        (
            "dingtalk",
            serde_json::json!({"url": url, "messageType": "actionCard", "template": {"title": "{{subject}}", "text": "{{body}}", "singleTitle": "Open", "singleURL": "https://example.com/{{resourceId}}"}}),
        ),
        (
            "dingtalk",
            serde_json::json!({"url": url, "messageType": "feedCard", "template": {"links": [{"title":"{{subject}}","messageURL":"https://example.com/{{resourceId}}"}]}}),
        ),
        (
            "wechat_work",
            serde_json::json!({"url": url, "messageType": "text", "mentionedList": ["@all"], "template": {"content": "{{subject}} {{body}}"}}),
        ),
        (
            "wechat_work",
            serde_json::json!({"url": url, "messageType": "markdown", "template": {"content": "### {{subject}}\n{{body}}"}}),
        ),
        (
            "wechat_work",
            serde_json::json!({"url": url, "messageType": "markdown_v2", "template": {"content": "# {{subject}}\n{{body}}"}}),
        ),
        (
            "wechat_work",
            serde_json::json!({"url": url, "messageType": "image", "template": {"base64": "BASE64-{{resourceId}}", "md5": "MD5-{{messageId}}"}}),
        ),
        (
            "wechat_work",
            serde_json::json!({"url": url, "messageType": "file", "template": {"media_id": "{{resourceId}}-file"}}),
        ),
        (
            "pagerduty",
            serde_json::json!({"url": url, "routingKey": "route-123", "messageType": "trigger", "template": {"summary": "{{subject}}", "dedupKey": "{{dedupeKey}}", "customDetails": {"body": "{{body}}"}}}),
        ),
        (
            "pagerduty",
            serde_json::json!({"url": url, "routingKey": "route-123", "messageType": "acknowledge", "template": {"dedupKey": "ack-{{messageId}}"}}),
        ),
        (
            "webhook",
            serde_json::json!({"url": url, "messageType": "json", "template": {"body": "{\"text\":\"{{subject}}\",\"status\":\"{{severity}}\"}"}}),
        ),
    ];

    for (provider, config) in cases {
        let result = client
            .deliver(
                &NotificationChannelDeliveryConfig {
                    id: format!("channel-{provider}"),
                    provider: provider.to_owned(),
                    enabled: true,
                    config_json: config.to_string(),
                    secret_refs_json: "{}".to_owned(),
                    target_redacted: "local".to_owned(),
                    safety_policy_json: Some(
                        serde_json::json!({"allowInsecureLoopback": true}).to_string(),
                    ),
                },
                &message,
            )
            .await;
        assert!(result.delivered, "{provider} should deliver: {result:?}");
    }

    let payloads = received.lock().await.clone();
    assert_eq!(payloads[0]["attachments"][0]["text"], "Exited 2");
    assert_eq!(payloads[1]["text"]["content"], "Job failed Exited 2");
    assert_eq!(
        payloads[2]["link"]["messageUrl"],
        "https://example.com/billing-nightly"
    );
    assert_eq!(
        payloads[3]["actionCard"]["singleURL"],
        "https://example.com/billing-nightly"
    );
    assert_eq!(payloads[4]["feedCard"]["links"][0]["title"], "Job failed");
    assert_eq!(payloads[5]["text"]["content"], "Job failed Exited 2");
    assert_eq!(payloads[5]["mentioned_list"][0], "@all");
    assert_eq!(
        payloads[6]["markdown"]["content"],
        "### Job failed\nExited 2"
    );
    assert_eq!(
        payloads[7]["markdown_v2"]["content"],
        "# Job failed\nExited 2"
    );
    assert_eq!(payloads[8]["image"]["base64"], "BASE64-billing-nightly");
    assert_eq!(payloads[9]["file"]["media_id"], "billing-nightly-file");
    assert_eq!(payloads[10]["event_action"], "trigger");
    assert_eq!(
        payloads[10]["payload"]["custom_details"]["body"],
        "Exited 2"
    );
    assert_eq!(payloads[11]["event_action"], "acknowledge");
    assert_eq!(payloads[11]["dedup_key"], "ack-msg-1");
    assert_eq!(payloads[12]["text"], "Job failed");
    assert_eq!(payloads[12]["status"], "critical");
    server.abort();
}

#[tokio::test]
async fn provider_delivery_accepts_drawer_template_strings_and_signs_office_bots() {
    let received = std::sync::Arc::new(tokio::sync::Mutex::new(
        Vec::<(String, serde_json::Value)>::new(),
    ));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap_or_else(|error| panic!("webhook listener should bind: {error}"));
    let url = format!(
        "http://{}/notify",
        listener
            .local_addr()
            .unwrap_or_else(|error| panic!("listener addr should read: {error}"))
    );
    let received_for_route = received.clone();
    let app = axum::Router::new().route(
        "/notify",
        axum::routing::post(
            move |uri: axum::http::Uri, axum::Json(payload): axum::Json<serde_json::Value>| {
                let received = received_for_route.clone();
                async move {
                    received.lock().await.push((uri.to_string(), payload));
                    axum::http::StatusCode::OK
                }
            },
        ),
    );
    let server = tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .unwrap_or_else(|error| panic!("webhook server should run: {error}"));
    });

    let message = sample_notification_message();
    let client = NotificationProviderClient::new(AlertDeliveryPolicy {
        allow_insecure_loopback: true,
    });

    for (provider, config, secrets) in [
        (
            "slack",
            serde_json::json!({
                "url": url,
                "messageType": "blockKit",
                "template": {
                    "text": "{{subject}}",
                    "blocks": r#"[{"type":"section","text":{"type":"mrkdwn","text":"{{body}} / {{severity}}"}}]"#
                }
            }),
            serde_json::json!({}),
        ),
        (
            "dingtalk",
            serde_json::json!({
                "url": url,
                "messageType": "markdown",
                "atMobiles": ["13800138000"],
                "isAtAll": false,
                "template": {"title": "{{subject}}", "text": "{{body}} {{eventType}}"}
            }),
            serde_json::json!({"signingKey": "env:PATH"}),
        ),
        (
            "feishu",
            serde_json::json!({
                "url": url,
                "messageType": "interactive",
                "template": {
                    "card": r#"{"header":{"title":{"tag":"plain_text","content":"{{subject}}"}},"elements":[{"tag":"div","text":{"tag":"lark_md","content":"{{body}}"}}]}"#
                }
            }),
            serde_json::json!({"signingKey": "env:PATH"}),
        ),
        (
            "wechat_work",
            serde_json::json!({
                "url": url,
                "messageType": "template_card",
                "template": {
                    "templateCard": r#"{"card_type":"text_notice","main_title":{"title":"{{subject}}","desc":"{{body}}"},"card_action":{"type":1,"url":"https://example.com/{{resourceId}}"}}"#
                }
            }),
            serde_json::json!({}),
        ),
    ] {
        let result = client
            .deliver(
                &NotificationChannelDeliveryConfig {
                    id: format!("channel-{provider}"),
                    provider: provider.to_owned(),
                    enabled: true,
                    config_json: config.to_string(),
                    secret_refs_json: secrets.to_string(),
                    target_redacted: "local".to_owned(),
                    safety_policy_json: Some(
                        serde_json::json!({"allowInsecureLoopback": true}).to_string(),
                    ),
                },
                &message,
            )
            .await;
        assert!(result.delivered, "{provider} should deliver: {result:?}");
    }

    let payloads = received.lock().await.clone();
    assert_eq!(
        payloads[0].1["blocks"][0]["text"]["text"],
        "Exited 2 / critical"
    );
    assert!(payloads[1].0.contains("timestamp="));
    assert!(payloads[1].0.contains("sign="));
    assert_eq!(payloads[1].1["at"]["atMobiles"][0], "13800138000");
    assert!(payloads[2].1.get("timestamp").is_some());
    assert!(payloads[2].1.get("sign").is_some());
    assert_eq!(
        payloads[2].1["card"]["header"]["title"]["content"],
        "Job failed"
    );
    assert_eq!(payloads[3].1["msgtype"], "template_card");
    assert_eq!(
        payloads[3].1["template_card"]["main_title"]["desc"],
        "Exited 2"
    );
    server.abort();
}

#[test]
fn email_template_config_overrides_alert_payload_subject_and_body() {
    let message = sample_notification_message();
    let config = parse_json_object(
        &serde_json::json!({
            "template": {
                "subject": "{{subject}} / {{severity}}",
                "body": "{{body}} / {{eventType}}",
                "html": "<strong>{{body}}</strong>"
            }
        })
        .to_string(),
    );
    let payload = email_alert_payload_from_message(&message, &config);

    assert_eq!(payload.rule_name, "Job failed / critical");
    assert_eq!(payload.message, "Exited 2 / job_instance.failed");
    assert_eq!(payload.resource_type, "job");
    assert_eq!(config["template"]["html"], "<strong>{{body}}</strong>");
}

fn sample_notification_message() -> NotificationMessageSummary {
    NotificationMessageSummary {
        id: "msg-1".to_owned(),
        source_type: "job_instance".to_owned(),
        source_id: "instance-1".to_owned(),
        policy_id: "policy-1".to_owned(),
        event_type: "job_instance.failed".to_owned(),
        resource_type: "job".to_owned(),
        resource_id: "billing-nightly".to_owned(),
        severity: "critical".to_owned(),
        subject: "Job failed".to_owned(),
        body: "Exited 2".to_owned(),
        payload_json: serde_json::json!({"eventType":"job_instance.failed"}).to_string(),
        dedupe_key: "policy-1:instance-1:failed".to_owned(),
        trace_id: None,
        status: "pending".to_owned(),
        created_at: "2026-06-11T00:00:00Z".to_owned(),
        updated_at: "2026-06-11T00:00:00Z".to_owned(),
    }
}
