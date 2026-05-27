use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::HeaderMap,
};
use tikee_core::{ExecutionMode, TriggerType};
use tikee_storage::{AppendJobInstanceLog, CreateJobInstance};

use crate::http::{
    AppState, auth,
    dto::{
        ApiResponse, ErrorResponse, InboundWebhookTriggerApiResponse, InboundWebhookTriggerRequest,
        InboundWebhookTriggerResponse,
    },
    error::ApiError,
};

/// Trigger a job from an inbound webhook/event-source payload.
///
/// # Errors
///
/// Returns authorization, not-found, or storage errors when trigger creation fails.
#[utoipa::path(
    post,
    path = "/api/v1/events/webhooks/{job}:trigger",
    tag = "event-sources",
    params(("job" = String, Path, description = "Job identifier")),
    request_body = InboundWebhookTriggerRequest,
    responses(
        (status = 200, description = "Accepted inbound webhook event", body = InboundWebhookTriggerApiResponse),
        (status = 404, description = "Job not found", body = ErrorResponse),
        (status = 500, description = "Storage error", body = ErrorResponse)
    )
)]
pub async fn trigger_inbound_webhook(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(job_action): Path<String>,
    Json(request): Json<InboundWebhookTriggerRequest>,
) -> Result<Json<InboundWebhookTriggerApiResponse>, ApiError> {
    let principal = auth::require_permission(&headers, &state, "instances", "execute").await?;
    let job = job_action
        .strip_suffix(":trigger")
        .filter(|job| !job.is_empty())
        .map(ToOwned::to_owned)
        .ok_or_else(|| {
            ApiError::not_found(format!("unsupported webhook job action: {job_action}"))
        })?;
    let job_summary = state
        .jobs
        .get(&job)
        .await
        .map_err(|error| ApiError::storage(&error))?
        .ok_or_else(|| ApiError::not_found(format!("job not found: {job}")))?;
    if !crate::http::access_scope::allows_resource(
        &principal.scope_bindings,
        &job_summary.namespace,
        &job_summary.app,
        None,
    ) {
        return Err(ApiError::forbidden(
            "api token scope binding does not allow this namespace/app",
        ));
    }
    let instance = state
        .instances
        .create_pending(CreateJobInstance {
            job_id: job.clone(),
            trigger_type: TriggerType::Webhook,
            execution_mode: ExecutionMode::Single,
        })
        .await
        .map_err(|error| ApiError::storage(&error))?
        .ok_or_else(|| ApiError::not_found(format!("job not found: {job}")))?;

    append_webhook_log(&state, &instance.id, &request).await?;

    Ok(Json(ApiResponse::success(InboundWebhookTriggerResponse {
        accepted: true,
        instance_id: instance.id,
        job_id: instance.job_id,
        status: instance.status.to_string(),
        trigger_type: instance.trigger_type.to_string(),
    })))
}

async fn append_webhook_log(
    state: &AppState,
    instance_id: &str,
    request: &InboundWebhookTriggerRequest,
) -> Result<(), ApiError> {
    let source = request
        .source
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("webhook");
    let event_type = request
        .event_type
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("webhook.event");
    let payload = request.payload.clone().unwrap_or(serde_json::Value::Null);
    let message = serde_json::json!({
        "event": "webhook_event_source",
        "source": source,
        "event_type": event_type,
        "payload": payload,
    });
    state
        .logs
        .append(AppendJobInstanceLog {
            instance_id: instance_id.to_owned(),
            worker_id: format!("event-source:{source}"),
            level: "info".to_owned(),
            message: message.to_string(),
            sequence: 0,
        })
        .await
        .map_err(|error| ApiError::storage(&error))?;
    Ok(())
}
