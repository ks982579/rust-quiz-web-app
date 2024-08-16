//! frontend/src/main.rs
//! Entry point of application
use frontend::app::*;
use leptos::*;

/// Entrypoint of application
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
