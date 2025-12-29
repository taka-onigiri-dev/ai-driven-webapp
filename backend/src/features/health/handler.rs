use actix_web::{get, web, HttpResponse};
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    timestamp: String,
}

/// ヘルスチェックエンドポイント
///
/// システムの稼働状態を確認します。
#[get("/api/v1/health")]
async fn health_check() -> HttpResponse {
    let response = HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Utc::now().to_rfc3339(),
    };

    HttpResponse::Ok().json(response)
}

/// ルーティング設定
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check);
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(App::new().configure(configure)).await;

        let req = test::TestRequest::get()
            .uri("/api/v1/health")
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        assert_eq!(resp.status(), 200);
    }

    #[actix_web::test]
    async fn test_health_check_response_format() {
        let app = test::init_service(App::new().configure(configure)).await;

        let req = test::TestRequest::get()
            .uri("/api/v1/health")
            .to_request();

        let resp = test::call_service(&app, req).await;
        let body: HealthResponse = test::read_body_json(resp).await;

        assert_eq!(body.status, "ok");
        assert_eq!(body.version, env!("CARGO_PKG_VERSION"));
        assert!(!body.timestamp.is_empty());
    }
}
