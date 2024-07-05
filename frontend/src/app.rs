//! frontend/src/app.rs
//! This holds the main app component for this web UI
use crate::router::AppRouter;
use crate::store::AppSettings;
use crate::store::AuthState;
use leptos::*;
use leptos_dom::logging::console_log;

#[component]
pub fn App() -> impl IntoView {
    console_log("Opening App Component");
    // Adding Context
    let auth_state = AuthState::new();
    provide_context(auth_state.clone());
    console_log("Creating App Settings");
    let app_settings = AppSettings::init();
    provide_context(app_settings.clone());
    console_log("AppSettings created");

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
