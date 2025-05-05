use crate::handler::ApiResult;

use crate::state::AppState;
use actix_web::{web, web::Json, Responder};
use chrono::Local;
use mongodb::bson::{doc, Document};
use mongodb::Collection;
use crate::model::auth::LoginResponse;

pub async fn login(app_state: web::Data<AppState>) -> impl Responder {
    let now = Local::now();
    let now_str = format!(
        "Login ok! Server current time is: {}",
        now.format("%Y-%m-%d %H:%M:%S")
    );
    let users: Collection<Document> = app_state.mongo_db.collection("users");
    let user = users.find_one(doc! {"email":"alice@example.com"}).await;

    match user {
        Ok(user) => {
            let response = LoginResponse { user, now_str };
            Json(ApiResult::success(response))
        }
        Err(e) => Json(ApiResult::error(e.to_string())),
    }
}

pub async fn logout() -> impl Responder {
    let now = Local::now();
    let now_str = format!(
        "Logout ok! Server current time is: {}",
        now.format("%Y-%m-%d %H:%M:%S")
    );
    Json(ApiResult::success(now_str))
}
