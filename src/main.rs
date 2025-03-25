use std::env;

use crate::route::auth::auth_routes;
use crate::route::index::general_routes;
use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};

#[path = "./handler/mod.rs"]
mod handler;
#[path = "./route/mod.rs"]
mod route;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("RSCMS_SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("RSCMS_SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let port_num = port.parse::<u16>().expect("Invalid port number");
    let mongo_uri = env::var("RSCMS_MONGODB_URI");
    if mongo_uri.is_err() {
        panic!("MongoDB URL not set!");
    }

    // 连接 MongoDB 数据库
    let mongo_uri_str = mongo_uri.unwrap();
    let client_options = ClientOptions::parse(&mongo_uri_str)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let client = Client::with_options(client_options)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let db = client.database("rscms");

    // 共享 MongoDB 数据库实例
    let db_instance = web::Data::new(db);

    let app = move || {
        App::new()
            .configure(general_routes)
            .configure(auth_routes)
            .app_data(db_instance.clone())
            .default_service(
                web::route().to(|| async { HttpResponse::NotFound().body("404 Not Found") }),
            )
    };
    let server = HttpServer::new(app)
        .shutdown_timeout(120)
        .bind((host.as_str(), port_num))
        .expect(&format!("Can not bind to port {}:{}", host, port));

    println!("Server running at http://{}:{}/", host, port);

    server.run().await
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
