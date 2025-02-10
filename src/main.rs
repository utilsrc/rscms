use std::env;

use crate::route::auth::auth_routes;
use crate::route::index::general_routes;
use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;

#[path = "./handler/mod.rs"]
mod handler;
#[path = "./route/mod.rs"]
mod route;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mongo_url = env::var("RSCMS_MONGODB_URL");
    if mongo_url.is_err() {
        panic!("MongoDB URL not set!");
    }
    println!("MongoDB URL: {}", mongo_url.unwrap());

    let app = move || {
        App::new()
            .configure(general_routes)
            .configure(auth_routes)
            .default_service(
                web::route().to(|| async { HttpResponse::NotFound().body("404 Not Found") }),
            )
    };
    HttpServer::new(app)
        .shutdown_timeout(120)
        .bind(("0.0.0.0", 8080))
        .expect("Can not bind to port 8080")
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use bcrypt::{hash, verify, DEFAULT_COST};

    #[test]
    fn test_default_password_match() {
        let passwd = "rscms-admin";
        let hashed = hash(passwd, DEFAULT_COST).unwrap();
        println!("hashed: {}", hashed);

        let valid = verify(passwd, &hashed).unwrap();
        assert!(valid);
    }
}
