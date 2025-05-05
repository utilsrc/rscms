use serde::Serialize;

pub mod auth;

#[derive(Serialize)]
pub struct ApiResult<T: Serialize> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResult<T> {
    pub fn success(data: T) -> Self {
        ApiResult {
            code: 0,
            msg: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn error<E: ToString>(err: E) -> Self {
        ApiResult {
            code: 1,
            msg: err.to_string(),
            data: None,
        }
    }
}
