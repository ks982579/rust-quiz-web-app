//! frontend/src/main.rs
use leptos::*;

use frontend::app::*;
use frontend::utils::*;

/// Entrypoint of application
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
