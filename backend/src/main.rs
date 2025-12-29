mod config;
mod shared;
mod features;
mod entities;

use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_cors::Cors;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 環境変数の読み込み
    dotenv::dotenv().ok();

    // ログの初期化
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 設定の読み込み
    let config = config::Config::from_env();
    let host = config.server.host.clone();
    let port = config.server.port;

    tracing::info!("Starting server at {}:{}", host, port);

    // データベース接続プールの作成
    let db = config::db::create_connection_pool(&config.database.url).await
        .expect("Failed to create database connection pool");

    // HTTPサーバーの起動
    HttpServer::new(move || {
        // CORS設定
        let cors = Cors::permissive(); // TODO: 本番環境では適切に設定する

        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(config.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .configure(features::health::configure)
            .configure(features::auth::configure)
            // 他のfeatureはここに追加
    })
    .bind((host, port))?
    .run()
    .await
}
