use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::HeaderMap,
};

use crate::http::{
    AppState, auth,
    dto::{
        ApiResponse, ErrorResponse, JobSchedulingAdviceApiResponse, JobSchedulingAdviceResponse,
    },
    error::ApiError,
};

/// Return operator-facing scheduling readiness advice for one job.
///
/// # Errors
///
/// Returns authorization, not-found, or storage errors when advice inputs cannot be loaded.
#[utoipa::path(
    get,
    path = "/api/v1/jobs/{job}/scheduling-advice",
    tag = "jobs",
    params(("job" = String, Path, description = "Job identifier")),
    responses(
        (status = 200, description = "Job scheduling advice", body = JobSchedulingAdviceApiResponse),
        (status = 404, description = "Job not found", body = ErrorResponse),
        (status = 500, description = "Storage error", body = ErrorResponse)
    )
)]
pub async fn job_scheduling_advice(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(job): Path<String>,
) -> Result<Json<JobSchedulingAdviceApiResponse>, ApiError> {
    let principal = auth::require_permission(&headers, &state, "jobs", "read").await?;
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
    let required_capability = required_capability_for_job(&job_summary);
    let eligible_workers = state
        .registry
        .find_eligible_workers_with_capability(
            &job_summary.namespace,
            &job_summary.app,
            required_capability.as_deref(),
        )
        .await;
    let instances = state
        .instances
        .list_by_job(&job_summary.id)
        .await
        .map_err(|error| ApiError::storage(&error))?;
    let recent_instances = instances.len().min(20);
    let recent_failures = instances
        .iter()
        .take(20)
        .filter(|instance| instance.status.to_string() == "failed")
        .count();
    let (ready, severity, reason) = advice_status(&eligible_workers, recent_failures);

    Ok(Json(ApiResponse::success(JobSchedulingAdviceResponse {
        ready,
        severity,
        reason,
        required_capability,
        eligible_workers,
        recent_instances: u64::try_from(recent_instances).unwrap_or(u64::MAX),
        recent_failures: u64::try_from(recent_failures).unwrap_or(u64::MAX),
    })))
}

fn required_capability_for_job(job: &tikee_storage::JobSummary) -> Option<String> {
    if job
        .script_id
        .as_deref()
        .is_some_and(|value| !value.trim().is_empty())
    {
        return Some("script".to_owned());
    }
    let processor = job
        .processor_name
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(&job.name);
    Some(format!("processor:{processor}"))
}

fn advice_status(eligible_workers: &[String], recent_failures: usize) -> (bool, String, String) {
    if eligible_workers.is_empty() {
        return (
            false,
            "error".to_owned(),
            "no online worker advertises the required capability".to_owned(),
        );
    }
    if recent_failures > 0 {
        return (
            true,
            "warning".to_owned(),
            format!(
                "{} eligible worker(s), but recent failures exist",
                eligible_workers.len()
            ),
        );
    }
    (
        true,
        "ok".to_owned(),
        format!("{} eligible worker(s) online", eligible_workers.len()),
    )
}
