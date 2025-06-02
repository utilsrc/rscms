use actix_web::web;

use crate::handler::auth::{login, logout, register};

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/token", web::post().to(login))
            .route("/token", web::delete().to(logout)),
    );
}
