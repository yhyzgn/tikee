//! HTTP error mapping.

use axum::{Json, http::StatusCode, response::IntoResponse};

use super::dto::ProblemDetails;

/// API error variants returned by management handlers.
#[derive(Debug, Clone)]
pub enum ApiError {
    /// The requested operation is part of the API contract but not implemented yet.
    NotImplemented {
        /// Human-readable not implemented reason.
        message: String,
    },
}

impl ApiError {
    const fn status_code(&self) -> StatusCode {
        match self {
            Self::NotImplemented { .. } => StatusCode::NOT_IMPLEMENTED,
        }
    }

    const fn code(&self) -> &'static str {
        match self {
            Self::NotImplemented { .. } => "not_implemented",
        }
    }

    fn message(&self) -> String {
        match self {
            Self::NotImplemented { message } => message.clone(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();
        let body = ProblemDetails {
            code: self.code(),
            message: self.message(),
            trace_id: "unavailable".to_owned(),
            details: None,
        };

        (status, Json(body)).into_response()
    }
}
