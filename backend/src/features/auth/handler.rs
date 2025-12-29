use super::domain::{LoginRequest, RefreshRequest, RegisterRequest};
use super::infra::{RefreshTokenRepositoryImpl, UserRepositoryImpl};
use super::service::AuthService;
use crate::config::Config;
use actix_web::{post, web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use validator::Validate;

#[post("/register")]
async fn register(
    db: web::Data<Arc<DatabaseConnection>>,
    config: web::Data<Config>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    // Validate request
    if let Err(e) = req.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "VALIDATION_ERROR",
            "message": e.to_string()
        }));
    }

    // Create service
    let user_repo = Arc::new(UserRepositoryImpl::new(db.get_ref().clone()));
    let token_repo = Arc::new(RefreshTokenRepositoryImpl::new(db.get_ref().clone()));
    let service = AuthService::new(user_repo, token_repo, config.get_ref().clone());

    // Execute
    match service.register(req.into_inner()).await {
        Ok(response) => HttpResponse::Created().json(response),
        Err(e) => e.error_response(),
    }
}

#[post("/login")]
async fn login(
    db: web::Data<Arc<DatabaseConnection>>,
    config: web::Data<Config>,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    // Validate request
    if let Err(e) = req.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "VALIDATION_ERROR",
            "message": e.to_string()
        }));
    }

    // Create service
    let user_repo = Arc::new(UserRepositoryImpl::new(db.get_ref().clone()));
    let token_repo = Arc::new(RefreshTokenRepositoryImpl::new(db.get_ref().clone()));
    let service = AuthService::new(user_repo, token_repo, config.get_ref().clone());

    // Execute
    match service.login(req.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => e.error_response(),
    }
}

#[post("/logout")]
async fn logout(
    db: web::Data<Arc<DatabaseConnection>>,
    config: web::Data<Config>,
    // TODO: Extract user_id from JWT in authorization header
) -> impl Responder {
    // For now, return success
    // In production, you'd extract user_id from the authorization header
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Logged out successfully"
    }))
}

#[post("/refresh")]
async fn refresh(
    db: web::Data<Arc<DatabaseConnection>>,
    config: web::Data<Config>,
    req: web::Json<RefreshRequest>,
) -> impl Responder {
    // Create service
    let user_repo = Arc::new(UserRepositoryImpl::new(db.get_ref().clone()));
    let token_repo = Arc::new(RefreshTokenRepositoryImpl::new(db.get_ref().clone()));
    let service = AuthService::new(user_repo, token_repo, config.get_ref().clone());

    // Execute
    match service.refresh(req.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => e.error_response(),
    }
}
