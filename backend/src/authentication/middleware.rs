//! backend/src/authentication/middleware.rs
//! To handle the middleware authentication with cookies
//! Following [Actix-web docs](https://docs.rs/actix-web/latest/actis_web/middleware/index.html)
//! closely and hoping for the best

use crate::session_wrapper::SessionWrapper;
use actix_session::SessionExt;
use actix_web::{
    body::{BoxBody, MessageBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::StatusCode,
    HttpMessage, HttpResponse,
};
use models::UserID;
use std::task::{Context, Poll};
use std::{boxed::Box, pin::Pin};

/// Returns HTTP Status 500 and preserves root cause for logging
pub fn http_500<T>(err: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    actix_web::error::ErrorInternalServerError(err)
}

pub type AuthInfo<A> = std::rc::Rc<A>;
type ActixError = actix_web::error::Error;
pub struct AuthCookie;

/// Implement TRAMSFORM
impl<S, B> Transform<S, ServiceRequest> for AuthCookie
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = ActixError;
    type InitError = ();
    type Transform = AuthCookieMiddleware<S>;
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(AuthCookieMiddleware { service }))
    }
}

pub struct AuthCookieMiddleware<S> {
    service: S,
}

pub type LocalBoxFuture<'a, T> = Pin<Box<dyn std::future::Future<Output = T> + 'a>>;

/// Implement SERVICE
impl<S, B> Service<ServiceRequest> for AuthCookieMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
    B: 'static + MessageBody,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    // Believe this is the required `poll_ready()` implementation
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // println!("Hi from start. You requested: {}", req.path());

        // Creating our Session Wrapper
        // if let Some(session) = http_request.get_session() {
        let this_session = SessionWrapper::wrap(req.get_session());
        let user_id_res = this_session.get_user_id(); //.map(Rc::new);

        // If there's an error, we say it is Internal Server Error
        let user_id = if let Ok(id) = user_id_res {
            id
        } else {
            let (http_req, _) = req.into_parts();
            let unauth_response = HttpResponse::InternalServerError().finish();
            let service_response = ServiceResponse::new(http_req, unauth_response);
            return Box::pin(async {
                // After much fighting with borrow checker this is what works best
                // forget the original requestion and return a clean slate
                let _err = anyhow::anyhow!("Internal Error");
                Ok(service_response)
            });
        };

        if let Some(userid) = user_id {
            req.extensions_mut().insert(UserID(userid.to_string()));
            let req_fut = self.service.call(req);
            Box::pin(async move {
                let res = req_fut.await?;
                Ok(res.map_into_boxed_body())
            })
        } else {
            let (http_req, _) = req.into_parts();
            let unauth_response = HttpResponse::Unauthorized().finish();
            let service_response = ServiceResponse::new(http_req, unauth_response);
            Box::pin(async {
                // After much fighting with borrow checker this is what works best
                // forget the original requestion and return a clean slate
                let _err = anyhow::anyhow!("User not logged in");
                Ok(service_response)
                // Err(actix_web::error::InternalError::from_response(
                //     err,
                //     HttpResponse::Unauthorized().finish(),
                // )
                // .into())
                // Err(actix_web::error::InternalError::new(err, StatusCode::UNAUTHORIZED).into())
            })
        }
    }
}
