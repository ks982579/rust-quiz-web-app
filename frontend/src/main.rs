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
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
