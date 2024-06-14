//! frontend/src/main.rs
use leptos::*;

mod app;
mod pages;
mod router;

use crate::app::*;

/// Entrypoint of application
fn main() {
    mount_to_body(App)
}
