//! frontend/src/app.rs
//! This holds the main app component for this web UI
use crate::router::AppRouter;
use crate::store::AppSettings;
use crate::store::AuthState;
use leptos::*;

/// Main component for rendering entire application
#[component]
pub fn App() -> impl IntoView {
    // Adding Context
    let auth_state = AuthState::new();
    provide_context(auth_state.clone());
    let app_settings = AppSettings::init();
    provide_context(app_settings.clone());

    view! {
        <AppRouter />
    }
}
