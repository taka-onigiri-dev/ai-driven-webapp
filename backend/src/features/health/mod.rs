use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}
