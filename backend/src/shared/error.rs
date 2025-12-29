use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

/// アプリケーション全体で使用するResult型
pub type AppResult<T> = Result<T, AppError>;

/// アプリケーションエラー型
#[derive(Debug)]
pub enum AppError {
    /// バリデーションエラー
    ValidationError { field: String, message: String },

    /// 認証エラー
    Unauthorized { message: String },

    /// 認可エラー（権限不足）
    Forbidden { message: String },

    /// リソース不存在
    NotFound { resource: String, id: String },

    /// 競合エラー（一意制約違反等）
    Conflict { message: String },

    /// ビジネスルール違反
    BusinessRuleViolation { message: String },

    /// データベースエラー
    DatabaseError { message: String },

    /// 内部エラー
    InternalError { message: String },
}

impl AppError {
    /// エラーコードを取得
    pub fn error_code(&self) -> &'static str {
        match self {
            AppError::ValidationError { .. } => "VALIDATION_ERROR",
            AppError::Unauthorized { .. } => "UNAUTHORIZED",
            AppError::Forbidden { .. } => "FORBIDDEN",
            AppError::NotFound { .. } => "NOT_FOUND",
            AppError::Conflict { .. } => "CONFLICT",
            AppError::BusinessRuleViolation { .. } => "BUSINESS_RULE_VIOLATION",
            AppError::DatabaseError { .. } => "DATABASE_ERROR",
            AppError::InternalError { .. } => "INTERNAL_ERROR",
        }
    }

    /// エラーメッセージを取得
    pub fn message(&self) -> String {
        match self {
            AppError::ValidationError { field, message } => {
                format!("Validation error in field '{}': {}", field, message)
            }
            AppError::Unauthorized { message } => message.clone(),
            AppError::Forbidden { message } => message.clone(),
            AppError::NotFound { resource, id } => {
                format!("{} with id '{}' not found", resource, id)
            }
            AppError::Conflict { message } => message.clone(),
            AppError::BusinessRuleViolation { message } => message.clone(),
            AppError::DatabaseError { message } => message.clone(),
            AppError::InternalError { message } => message.clone(),
        }
    }

    /// HTTPステータスコードを取得
    pub fn status_code(&self) -> StatusCode {
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
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for AppError {}

/// エラーレスポンス用の構造体
#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: ErrorDetail,
}

#[derive(Debug, Serialize)]
struct ErrorDetail {
    code: String,
    message: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        self.status_code()
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            error: ErrorDetail {
                code: self.error_code().to_string(),
                message: self.message(),
            },
        };

        // エラーをログに記録
        if status_code.is_server_error() {
            tracing::error!(
                error_code = self.error_code(),
                message = %self.message(),
                "Server error occurred"
            );
        } else {
            tracing::warn!(
                error_code = self.error_code(),
                message = %self.message(),
                "Client error occurred"
            );
        }

        HttpResponse::build(status_code).json(error_response)
    }
}

/// SeaORMのDbErrからの変換
impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        tracing::error!("Database error: {:?}", err);
        AppError::DatabaseError {
            message: "Database operation failed".to_string(),
        }
    }
}

/// anyhow::Errorからの変換
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        tracing::error!("Internal error: {:?}", err);
        AppError::InternalError {
            message: "An unexpected error occurred".to_string(),
        }
    }
}
