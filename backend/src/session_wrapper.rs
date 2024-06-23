//! backend/src/session_wrapper.rs
//! Wraps the `actix_session::Session` struct for customization
use std::future::{ready, Ready};

use actix_session::{Session, SessionExt, SessionGetError, SessionInsertError};
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use uuid::Uuid;

pub struct SessionWrapper(Session);

impl SessionWrapper {
    const USER_ID_KEY: &'static str = "user_id";
    pub fn wrap(session: Session) -> Self {
        Self(session)
    }
    pub fn renew(&self) {
        self.0.renew();
    }
    pub fn insert_user_id(&self, user_id: Uuid) -> Result<(), SessionInsertError> {
        self.0.insert(Self::USER_ID_KEY, user_id)
    }
    pub fn get_user_id(&self) -> Result<Option<Uuid>, SessionGetError> {
        self.0.get::<Uuid>(Self::USER_ID_KEY)
    }
    pub fn log_out(self) {
        self.0.purge()
    }
}

impl FromRequest for SessionWrapper {
    type Error = <Session as FromRequest>::Error;
    // To make non-future value into Future
    type Future = Ready<Result<SessionWrapper, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(Ok(SessionWrapper(req.get_session())))
    }
}
