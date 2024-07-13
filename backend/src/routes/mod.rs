//! backend/src/routes/mod.rs
mod create_questions;
mod create_quiz;
mod create_user;
mod get_question;
mod get_quiz;
mod health_check;
mod like_question;
mod login_user;
mod user_logout;

pub use create_questions::*;
pub use create_quiz::*;
pub use create_user::*;
pub use get_question::*;
pub use get_quiz::*;
pub use health_check::*;
pub use like_question::*;
pub use login_user::*;
pub use user_logout::*;
