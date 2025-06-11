use actix_web::{Error, FromRequest, HttpRequest, dev::ServiceRequest, dev::Service, dev::ServiceResponse};
use actix_web::error::ErrorUnauthorized;
use futures::future::{ready, Ready, LocalBoxFuture};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user id
    pub exp: usize,   // expiry timestamp
}

pub struct AuthMiddleware;

impl<S, B> actix_web::dev::Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> core::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req.headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "));

        let app_state = req.app_data::<actix_web::web::Data<crate::state::AppState>>().unwrap();

        match token {
            Some(token) => {
                match decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(app_state.jwt_secret.as_ref()),
                    &Validation::default(),
                ) {
                    Ok(_) => {
                        let fut = self.service.call(req);
                        Box::pin(async move {
                            fut.await
                        })
                    }
                    Err(_) => Box::pin(async move {
                        Err(ErrorUnauthorized("无效的token"))
                    }),
                }
            }
            None => Box::pin(async move {
                Err(ErrorUnauthorized("缺少认证token"))
            }),
        }
    }
}

impl FromRequest for Claims {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let token = req.headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "));

        let app_state = req.app_data::<actix_web::web::Data<crate::state::AppState>>().unwrap();

        match token {
            Some(token) => {
                match decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(app_state.jwt_secret.as_ref()),
                    &Validation::default(),
                ) {
                    Ok(token_data) => ready(Ok(token_data.claims)),
                    Err(_) => ready(Err(ErrorUnauthorized("无效的token"))),
                }
            }
            None => ready(Err(ErrorUnauthorized("缺少认证token"))),
        }
    }
}
