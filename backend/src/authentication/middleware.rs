//! backend/src/authentication/middleware.rs
//! To handle the middleware authentication with cookies
//! Following [Actix-web docs](https://docs.rs/actix-web/latest/actis_web/middleware/index.html)
//! closely and hoping for the best

use crate::session_wrapper::SessionWrapper;
use actix_session::SessionExt;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    HttpMessage, HttpResponse,
};
use models::UserID;
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
impl<NextServ, RespBody> Transform<NextServ, ServiceRequest> for AuthCookie
where
    NextServ: Service<ServiceRequest, Response = ServiceResponse<RespBody>, Error = ActixError>,
    NextServ::Future: 'static,
    RespBody: 'static,
{
    type Response = ServiceResponse<RespBody>;
    type Error = ActixError;
    type InitError = ();
    type Transform = AuthCookieMiddleware<NextServ>;
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: NextServ) -> Self::Future {
        std::future::ready(Ok(AuthCookieMiddleware { service }))
    }
}

pub struct AuthCookieMiddleware<S> {
    service: S,
}

pub type LocalBoxFuture<'a, T> = Pin<Box<dyn std::future::Future<Output = T> + 'a>>;

/// Implement SERVICE
impl<NextServ, RespBody> Service<ServiceRequest> for AuthCookieMiddleware<NextServ>
where
    NextServ: Service<ServiceRequest, Response = ServiceResponse<RespBody>, Error = ActixError>,
    NextServ::Future: 'static,
    RespBody: 'static,
{
    type Response = ServiceResponse<RespBody>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    // Believe this is the required `poll_ready()` implementation
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // println!("Hi from start. You requested: {}", req.path());

        // Creating our Session Wrapper
        // if let Some(session) = http_request.get_session() {
        let this_session = SessionWrapper::wrap(req.get_session());
        let user_id = this_session.get_user_id(); //.map(Rc::new);

        if let Some(userid) = user_id {
            req.extensions_mut().insert(UserID(userid.to_string()));
            let req_fut = self.service.call(req);
            Box::pin(async move {
                let res = req_fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(async move {
                // After much fighting with borrow checker this is what works best
                // forget the original requestion and return a clean slate
                let err = anyhow::anyhow!("User not logged in");
                Err(actix_web::error::InternalError::from_response(
                    err,
                    HttpResponse::Unauthorized().finish(),
                )
                .into())
            })
        }
    }
}
