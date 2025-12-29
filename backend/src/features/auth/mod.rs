mod domain;
mod handler;
mod infra;
mod repository;
mod service;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/auth")
            .service(handler::register)
            .service(handler::login)
            .service(handler::logout)
            .service(handler::refresh),
    );
}
