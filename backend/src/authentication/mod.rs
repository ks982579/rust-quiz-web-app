//! backend/src/authentication/mod.rs
//! backend/src/authentication/mod.rs
//! Holds logic and helpers related to authenticating users.
pub mod middleware;
pub mod password;

pub use middleware::*;
pub use password::*;
