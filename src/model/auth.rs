use mongodb::bson::Document;
use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponse {
    pub(crate) user: Option<Document>,
    pub(crate) now_str: String,
}