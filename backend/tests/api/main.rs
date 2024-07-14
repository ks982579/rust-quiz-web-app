//! backend/tests/api/main.rs
//! This structure will scope tests into a single test executable.
//! This makes it easier to share code and setup / execute tests
mod create_questions;
mod create_quiz;
mod create_user;
mod destroy_question;
mod destroy_quiz;
mod get_questions;
mod get_quiz;
mod health_check;
mod log_out_users;
mod loggin_user;
mod utils;
