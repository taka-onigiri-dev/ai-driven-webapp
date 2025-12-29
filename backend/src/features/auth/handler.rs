use super::domain::{LoginRequest, RefreshRequest, RegisterRequest};
use super::infra::{RefreshTokenRepositoryImpl, UserRepositoryImpl};
use super::service::AuthService;
use crate::config::Config;
use crate::shared::{ApiResponse, AppError, AppResult};
use actix_web::{post, web, HttpRequest, HttpResponse};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use validator::Validate;

/// ユーザー登録
#[post("/api/v1/auth/register")]
async fn register(
    req: web::Json<RegisterRequest>,
    db: web::Data<DatabaseConnection>,
    config: web::Data<Config>,
) -> AppResult<HttpResponse> {
    // バリデーション
    req.validate().map_err(|e| AppError::ValidationError {
        field: "request".to_string(),
        message: e.to_string(),
    })?;

    let service = create_auth_service(db.get_ref().clone(), config.get_ref().clone());
    let response = service.register(req.into_inner()).await?;

    Ok(HttpResponse::Created().json(ApiResponse::new(response)))
}

/// ログイン
#[post("/api/v1/auth/login")]
async fn login(
    req: web::Json<LoginRequest>,
    db: web::Data<DatabaseConnection>,
    config: web::Data<Config>,
) -> AppResult<HttpResponse> {
    // バリデーション
    req.validate().map_err(|e| AppError::ValidationError {
        field: "request".to_string(),
        message: e.to_string(),
    })?;

    let service = create_auth_service(db.get_ref().clone(), config.get_ref().clone());
    let response = service.login(req.into_inner()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(response)))
}

/// ログアウト
#[post("/api/v1/auth/logout")]
async fn logout(
    http_req: HttpRequest,
    db: web::Data<DatabaseConnection>,
    config: web::Data<Config>,
) -> AppResult<HttpResponse> {
    let service = create_auth_service(db.get_ref().clone(), config.get_ref().clone());

    // Authorizationヘッダーからトークンを取得
    let token = extract_token(&http_req)?;

    // トークンを検証してユーザーIDを取得
    let claims = service.verify_token(&token)?;

    // ログアウト処理
    service.logout(claims.sub).await?;

    Ok(HttpResponse::NoContent().finish())
}

/// トークンリフレッシュ
#[post("/api/v1/auth/refresh")]
async fn refresh(
    req: web::Json<RefreshRequest>,
    db: web::Data<DatabaseConnection>,
    config: web::Data<Config>,
) -> AppResult<HttpResponse> {
    let service = create_auth_service(db.get_ref().clone(), config.get_ref().clone());
    let response = service.refresh(req.into_inner()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(response)))
}

/// AuthServiceを作成するヘルパー関数
fn create_auth_service(db: DatabaseConnection, config: Config) -> AuthService {
    let user_repo = Arc::new(UserRepositoryImpl::new(db.clone()));
    let token_repo = Arc::new(RefreshTokenRepositoryImpl::new(db));
    AuthService::new(user_repo, token_repo, config)
}

/// Authorizationヘッダーからトークンを抽出
fn extract_token(req: &HttpRequest) -> AppResult<String> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized {
            message: "Missing authorization header".to_string(),
        })?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized {
            message: "Invalid authorization header format".to_string(),
        });
    }

    let token = auth_header.trim_start_matches("Bearer ").to_string();

    Ok(token)
}

/// ルーティング設定
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(register)
        .service(login)
        .service(logout)
        .service(refresh);
}
