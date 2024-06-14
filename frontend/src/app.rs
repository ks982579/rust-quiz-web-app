//! frontend/src/app.rs
//! This holds the main app component for this web UI
use crate::pages::*;
use crate::router::AppRouter;
use leptos::*;
use leptos_router::{Route, Router, Routes, A};

#[component]
pub fn App() -> impl IntoView {
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
