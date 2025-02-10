use actix_web::web;
use chrono::Local;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/",
        web::get().to(|| async {
            let now = Local::now();
            let now_str = format!(
                "healthy! Server current time is: {}",
                now.format("%Y-%m-%d %H:%M:%S")
            );
            now_str
        }),
    );
}
