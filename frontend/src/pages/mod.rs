//! frontend/src/pages/mod.rs
//! To export pages
mod dashboard;
mod home;
mod new_user;
mod protected_dashboard;

pub use dashboard::*;
pub use home::*;
pub use new_user::*;
pub use protected_dashboard::*;
