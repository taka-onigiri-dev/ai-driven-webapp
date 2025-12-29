use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    ValidationError { message: String },
    Unauthorized { message: String },
    Forbidden { message: String },
    NotFound { message: String },
    Conflict { message: String },
    BusinessRuleViolation { message: String },
    DatabaseError { message: String },
    InternalError { message: String },
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ValidationError { message } => write!(f, "Validation error: {}", message),
            AppError::Unauthorized { message } => write!(f, "Unauthorized: {}", message),
            AppError::Forbidden { message } => write!(f, "Forbidden: {}", message),
            AppError::NotFound { message } => write!(f, "Not found: {}", message),
            AppError::Conflict { message } => write!(f, "Conflict: {}", message),
            AppError::BusinessRuleViolation { message } => {
                write!(f, "Business rule violation: {}", message)
            }
            AppError::DatabaseError { message } => write!(f, "Database error: {}", message),
            AppError::InternalError { message } => write!(f, "Internal error: {}", message),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            AppError::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
            AppError::Forbidden { .. } => StatusCode::FORBIDDEN,
            AppError::NotFound { .. } => StatusCode::NOT_FOUND,
            AppError::Conflict { .. } => StatusCode::CONFLICT,
            AppError::BusinessRuleViolation { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::DatabaseError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error_code = match self {
            AppError::ValidationError { .. } => "VALIDATION_ERROR",
            AppError::Unauthorized { .. } => "UNAUTHORIZED",
            AppError::Forbidden { .. } => "FORBIDDEN",
            AppError::NotFound { .. } => "NOT_FOUND",
            AppError::Conflict { .. } => "CONFLICT",
            AppError::BusinessRuleViolation { .. } => "BUSINESS_RULE_VIOLATION",
            AppError::DatabaseError { .. } => "DATABASE_ERROR",
            AppError::InternalError { .. } => "INTERNAL_ERROR",
        };

        let message = match self {
            AppError::ValidationError { message } => message,
            AppError::Unauthorized { message } => message,
            AppError::Forbidden { message } => message,
            AppError::NotFound { message } => message,
            AppError::Conflict { message } => message,
            AppError::BusinessRuleViolation { message } => message,
            AppError::DatabaseError { message } => message,
            AppError::InternalError { message } => message,
        };

        if matches!(self, AppError::Unauthorized { .. }) {
            tracing::warn!(
                error_code = error_code,
                "Client error occurred"
            );
        } else if matches!(
            self,
            AppError::DatabaseError { .. } | AppError::InternalError { .. }
        ) {
            tracing::error!(
                error_code = error_code,
                error = %self,
                "Server error occurred"
            );
        }

        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error: error_code.to_string(),
            message: message.clone(),
        })
    }
}

impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        AppError::DatabaseError {
            message: err.to_string(),
        }
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        AppError::ValidationError {
            message: err.to_string(),
        }
    }
}
