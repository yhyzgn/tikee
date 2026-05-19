//! Minimal development authentication helpers for management APIs.

use axum::{Json, http::HeaderMap};

use super::{
    dto::{ApiResponse, AuthSession, LoginRequest, MeResponse},
    error::ApiError,
};

const DEFAULT_ADMIN_USERNAME: &str = "scheduler_init";
const DEFAULT_ADMIN_PASSWORD: &str = "Scheduler@2026!";
const DEFAULT_ADMIN_TOKEN: &str = "scheduler-init-token";
const ADMIN_ROLE: &str = "admin";

/// Validate a bearer token from request headers.
///
/// # Errors
///
/// Returns unauthorized when the header is missing, malformed, or does not match
/// the configured development admin token.
pub fn require_admin(headers: &HeaderMap) -> Result<(), ApiError> {
    let Some(value) = headers.get(axum::http::header::AUTHORIZATION) else {
        return Err(ApiError::unauthorized("missing bearer token"));
    };
    let Ok(value) = value.to_str() else {
        return Err(ApiError::unauthorized("invalid authorization header"));
    };
    let Some(token) = value.strip_prefix("Bearer ") else {
        return Err(ApiError::unauthorized(
            "authorization scheme must be Bearer",
        ));
    };
    if token == admin_token() {
        Ok(())
    } else {
        Err(ApiError::unauthorized("invalid bearer token"))
    }
}

/// Login with the development admin account.
///
/// # Errors
///
/// Returns unauthorized when credentials do not match the configured development account.
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Authenticated session", body = super::dto::LoginApiResponse),
        (status = 401, description = "Invalid credentials", body = super::dto::ErrorResponse)
    )
)]
pub async fn login(
    Json(request): Json<LoginRequest>,
) -> Result<Json<ApiResponse<AuthSession>>, ApiError> {
    if request.username == admin_username() && request.password == admin_password() {
        return Ok(Json(ApiResponse::success(AuthSession {
            token: admin_token(),
            username: admin_username(),
            roles: vec![ADMIN_ROLE.to_owned()],
        })));
    }

    Err(ApiError::unauthorized("invalid username or password"))
}

/// Return the current authenticated principal.
///
/// # Errors
///
/// Returns unauthorized when the bearer token is missing or invalid.
#[utoipa::path(
    get,
    path = "/api/v1/auth/me",
    tag = "auth",
    responses(
        (status = 200, description = "Current principal", body = super::dto::MeApiResponse),
        (status = 401, description = "Missing or invalid bearer token", body = super::dto::ErrorResponse)
    )
)]
pub async fn me(headers: HeaderMap) -> Result<Json<ApiResponse<MeResponse>>, ApiError> {
    require_admin(&headers)?;
    Ok(Json(ApiResponse::success(MeResponse {
        username: admin_username(),
        roles: vec![ADMIN_ROLE.to_owned()],
    })))
}

/// Logout endpoint for clients that want a uniform API call before dropping local tokens.
#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    tag = "auth",
    responses((status = 200, description = "Logout acknowledged", body = super::dto::EmptyApiResponse))
)]
pub async fn logout() -> Json<super::dto::EmptyApiResponse> {
    Json(ApiResponse::success(super::dto::EmptyData {}))
}

fn admin_username() -> String {
    std::env::var("SCHEDULER_DEV_ADMIN_USERNAME")
        .unwrap_or_else(|_| DEFAULT_ADMIN_USERNAME.to_owned())
}

fn admin_password() -> String {
    std::env::var("SCHEDULER_DEV_ADMIN_PASSWORD")
        .unwrap_or_else(|_| DEFAULT_ADMIN_PASSWORD.to_owned())
}

fn admin_token() -> String {
    std::env::var("SCHEDULER_DEV_ADMIN_TOKEN").unwrap_or_else(|_| DEFAULT_ADMIN_TOKEN.to_owned())
}
