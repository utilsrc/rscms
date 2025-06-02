use crate::handler::ApiResult;
use crate::model::auth::{LoginRequest, RegisterRequest, LoginResponse, TokenResponse, User};
use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use actix_web::{web, web::Json, Responder};
use bcrypt::verify;
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::bson::doc;
use mongodb::Collection;
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,  // user email
    exp: usize,   // expiry timestamp
}

pub async fn login(
    app_state: web::Data<AppState>,
    form: web::Json<LoginRequest>,
) -> impl Responder {
    let users: Collection<User> = app_state.mongo_db.collection("users");
    
    // 查找用户
    let user = match users.find_one(doc! {"email": &form.email}).await {
        Ok(Some(user)) => user,
        Ok(None) => return Json(ApiResult::error("用户不存在")),
        Err(e) => return Json(ApiResult::error(e.to_string())),
    };

    // 验证密码
    if !verify(&form.password, &user.password_hash).unwrap_or(false) {
        return Json(ApiResult::error("密码错误"));
    }

    // 生成JWT令牌
    let exp = (Utc::now() + chrono::Duration::hours(24)).timestamp() as usize;
    let claims = Claims {
        sub: user.email.clone(),
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    ).unwrap();

    let response = LoginResponse {
        user,
        token: TokenResponse {
            token,
            expires_in: 86400, // 24小时
        },
    };

    Json(ApiResult::success(response))
}

pub async fn logout() -> impl Responder {
    Json(ApiResult::success(json!({"message": "登出成功"})))
}

pub async fn register(
    app_state: web::Data<AppState>,
    form: web::Json<RegisterRequest>,
) -> impl Responder {
    let users: Collection<User> = app_state.mongo_db.collection("users");
    
    // 检查用户是否已存在
    if let Ok(Some(_)) = users.find_one(doc! {"email": &form.email}).await {
        return Json(ApiResult::error("用户已存在"));
    }

    // 加密密码
    let password_hash = hash(&form.password, DEFAULT_COST).unwrap();
    let now = Utc::now();

    // 创建用户
    let user = User {
        id: None,
        email: form.email.clone(),
        password_hash,
        created_at: now,
        updated_at: now,
    };

    match users.insert_one(user).await {
        Ok(_) => Json(ApiResult::success(json!({"message": "注册成功"}))),
        Err(e) => Json(ApiResult::error(e.to_string())),
    }
}
