use actix_web::web;

use crate::handler::auth::{login, logout};

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::get().to(login)) // correct route: post /auth/token
            .route("/logout", web::get().to(logout)), // correct route: delete /auth/token
    );
}
