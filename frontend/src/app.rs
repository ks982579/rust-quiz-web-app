//! frontend/src/app.rs
//! This holds the main app component for this web UI
use crate::router::AppRouter;
use crate::store::AppSettings;
use crate::{pages::*, store::AuthState};
use leptos::*;
use leptos_router::{Route, Router, Routes, A};

#[component]
pub fn App() -> impl IntoView {
    // Adding Context
    let auth_state = AuthState::new();
    provide_context(auth_state.clone());
    let app_settings = AppSettings::init();
    provide_context(app_settings.clone());

    view! {
        <>
            <header>
                <h1>"Kev's Quiz Web App"</h1>
                <nav>
                    <h3>"Just the Navbar section here"</h3>
                </nav>
            </header>
            <main>
                <AppRouter />
            </main>
            <footer>"&copy; 2024 Kev's Quiz Web App"</footer>
        </>
    }
}
