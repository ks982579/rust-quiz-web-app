//! backend/src/routes/mod.rs
mod create_quiz;
mod create_user;
mod health_check;
mod login_user;
mod user_logout;

pub use create_quiz::*;
pub use create_user::*;
pub use health_check::*;
pub use login_user::*;
pub use user_logout::*;
