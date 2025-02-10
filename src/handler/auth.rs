use crate::handler::ApiResult;

use actix_web::{web::Json, Responder};
use chrono::Local;

pub async fn login() -> impl Responder {
    let now = Local::now();
    let now_str = format!("Login ok! Server current time is: {}", now.format("%Y-%m-%d %H:%M:%S"));
    Json(ApiResult::success(now_str))
}

pub async fn logout() -> impl Responder {
    let now = Local::now();
    let now_str = format!("Logout ok! Server current time is: {}", now.format("%Y-%m-%d %H:%M:%S"));
    Json(ApiResult::success(now_str))
}