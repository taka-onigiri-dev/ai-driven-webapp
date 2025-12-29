mod config;
mod entities;
mod features;
mod shared;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use sea_orm::Database;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ai_webapp_backend=debug,actix_web=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    dotenv::dotenv().ok();
    let config = config::Config::from_env();

    // Database connection
    let db = Database::connect(&config.database.url)
        .await
        .expect("Failed to connect to database");
    let db = Arc::new(db);

    tracing::info!("Starting server at http://0.0.0.0:8080");

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(config.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .configure(features::health::configure)
            .configure(features::auth::configure)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
