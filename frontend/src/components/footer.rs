//! frontend/src/components/footer.rs
//! This is a file for the footer component(s) for consistency.
use leptos::*;

/// Component to render the footer consistently through application
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <div
                class:footer-links=true
            >
                <a href="https://techhub.social/@ks982579">Mastodon</a>
                <a href="https://github.com/ks982579">Github</a>
                <a href="https://www.linkedin.com/in/kevin-sullivan-a35a4964/">LinkedIn</a>
            </div>
            <div>"© 2024 Kev's Quiz Web App"</div>
        </footer>
    }
}
