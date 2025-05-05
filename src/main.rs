use std::env;

use crate::route::auth::auth_routes;
use crate::route::index::general_routes;
use crate::state::AppState;
use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use mongodb::{bson::doc, Client};

#[path = "./handler/mod.rs"]
mod handler;
#[path = "./route/mod.rs"]
mod route;
mod state;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // 读取环境变量
    let host = env::var("RSCMS_SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("RSCMS_SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let port_num = port.parse::<u16>().expect("Invalid port number");
    let mongo_uri = env::var("RSCMS_MONGODB_URI");
    let db_name = env::var("RSCMS_MONGODB_DB_NAME");
    if mongo_uri.is_err() {
        panic!("MongoDB URL not set!");
    }
    if db_name.is_err() {
        panic!("MongoDB database name not set!");
    }
    let mongo_uri = mongo_uri.unwrap();
    let db_name = db_name.unwrap();

    // 连接 MongoDB 数据库
    let client = Client::with_uri_str(mongo_uri).await.unwrap();
    let database = client.database(db_name.as_str());
    match database.run_command(doc! {"ping": 1}).await {
        Ok(_) => println!("✅ Successfully connected to MongoDB database: {}", db_name),
        Err(e) => panic!("❌ Failed to connect to MongoDB database: {}", e),
    }

    // 共享 MongoDB 数据库实例
    let shared_data = web::Data::new(AppState { mongo_db: database });
    let app = move || {
        App::new()
            .configure(general_routes)
            .configure(auth_routes)
            .app_data(shared_data.clone())
            .default_service(
                web::route().to(|| async { HttpResponse::NotFound().body("404 Not Found") }),
            )
    };

    // 启动服务器
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
        let passwd = "rscms";
        let hashed = hash(passwd, DEFAULT_COST).unwrap();
        println!("hashed: {}", hashed);

        let valid = verify(passwd, &hashed).unwrap();
        assert!(valid);
    }
}
