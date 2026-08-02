use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("not found")]
    NotFound,
    #[error("unauthorized")]
    Unauthorized,
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("forbidden")]
    Forbidden,
    #[error("validation error: {0}")]
    Validation(String),
    #[error("rate limited")]
    RateLimited,
    #[error("service unavailable")]
    Unavailable,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl AppError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotFound => "NOT_FOUND",
            Self::Unauthorized => "UNAUTHORIZED",
            Self::InvalidCredentials => "INVALID_CREDENTIALS",
            Self::Forbidden => "FORBIDDEN",
            Self::Validation(_) => "VALIDATION_ERROR",
            Self::RateLimited => "RATE_LIMITED",
            Self::Unavailable => "SERVICE_UNAVAILABLE",
            Self::Internal(_) => "INTERNAL_ERROR",
        }
    }

    pub fn status(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Unauthorized | Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::Validation(_) => StatusCode::BAD_REQUEST,
            Self::RateLimited => StatusCode::TOO_MANY_REQUESTS,
            Self::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn public_message(&self) -> String {
        match self {
            Self::Validation(message) => message.clone(),
            Self::NotFound => "Resource not found.".to_owned(),
            Self::Unauthorized => "Authentication required.".to_owned(),
            Self::InvalidCredentials => "Invalid credentials.".to_owned(),
            Self::Forbidden => "You do not have permission to perform this action.".to_owned(),
            Self::RateLimited => "Too many attempts. Try again later.".to_owned(),
            Self::Unavailable => "Service temporarily unavailable.".to_owned(),
            Self::Internal(_) => "An unexpected error occurred.".to_owned(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let request_id = Uuid::new_v4().to_string();
        if matches!(self, Self::Internal(_)) {
            tracing::error!(error = %self, %request_id, "internal error");
        }

        let body = json!({
            "error": {
                "code": self.code(),
                "message": self.public_message(),
                "request_id": request_id,
            }
        });

        (self.status(), Json(body)).into_response()
    }
}
