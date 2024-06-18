//! frontend/src/main.rs
use leptos::*;

mod app;
mod pages;
mod router;
mod store;
mod utils;

use crate::app::*;
use crate::utils::*;

/// Entrypoint of application
fn main() {
    mount_to_body(App)
}
