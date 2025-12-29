pub mod db;

use serde::Deserialize;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub cors: CorsConfig,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub access_token_expiry: i64,
    pub refresh_token_expiry: i64,
}

#[derive(Debug, Clone)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .expect("SERVER_PORT must be a valid number"),
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")
                    .expect("DATABASE_URL must be set"),
            },
            jwt: JwtConfig {
                secret: env::var("JWT_SECRET")
                    .expect("JWT_SECRET must be set"),
                access_token_expiry: env::var("JWT_ACCESS_TOKEN_EXPIRY")
                    .unwrap_or_else(|_| "900".to_string())
                    .parse()
                    .expect("JWT_ACCESS_TOKEN_EXPIRY must be a valid number"),
                refresh_token_expiry: env::var("JWT_REFRESH_TOKEN_EXPIRY")
                    .unwrap_or_else(|_| "604800".to_string())
                    .parse()
                    .expect("JWT_REFRESH_TOKEN_EXPIRY must be a valid number"),
            },
            cors: CorsConfig {
                allowed_origins: env::var("CORS_ALLOWED_ORIGINS")
                    .unwrap_or_else(|_| "http://localhost:3000".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
            },
        }
    }
}
