//! HTTP DTOs used by the management API.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Stable problem details response for HTTP errors.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ProblemDetails {
    /// Stable machine-readable error code.
    pub code: &'static str,
    /// Human-readable error message.
    pub message: String,
    /// Trace identifier used to correlate logs and client errors.
    pub trace_id: String,
    /// Optional structured error details.
    pub details: Option<serde_json::Value>,
}

/// Generic page response.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Page {
    /// Page items.
    pub items: Vec<JobSummary>,
    /// Token for the next page when more data is available.
    pub next_page_token: Option<String>,
}

/// Common list query parameters.
#[derive(Debug, Clone, Default, Deserialize, ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct PageQuery {
    /// Maximum number of items to return.
    pub page_size: Option<u32>,
    /// Opaque page token returned by a previous list call.
    pub page_token: Option<String>,
}

/// System information shown by the management API.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct SystemInfoResponse {
    /// API service name.
    pub name: &'static str,
    /// Server crate version.
    pub version: &'static str,
    /// Rust package target environment.
    pub target: &'static str,
}

/// Cluster status placeholder.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ClusterResponse {
    /// Cluster operating mode.
    pub mode: &'static str,
    /// Current node role.
    pub role: &'static str,
    /// Known server node count.
    pub nodes: u32,
}

/// Job summary DTO.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct JobSummary {
    /// Job identifier.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Job enabled flag.
    pub enabled: bool,
}

/// Create job request placeholder.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateJobRequest {
    /// Display name.
    pub name: String,
}
