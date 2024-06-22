//! frontend/src/pages/dashboard.rs
//! This is dashboard that appears for logged in users.
use leptos::*;
use models::{JsonMsg, PartialUser};

/// Dashboard component to be the main logged in part of homepage.
#[component]
pub fn Dashboard() -> impl IntoView {
    let user: PartialUser = use_context().expect("PartialUser Context not set");
    view! {
        <>
            <nav>"Shoule have own nav bar"</nav>
            <h1>"Welcome back "{user.name}</h1>
            <summary>"Heading for details tag"</summary>
            <details>"additional things user can open and close as needed."</details>
            <aside>"Content aside from content, like side bar!"</aside>
            <section>"Defines section in document"</section>
            <article>"Independent, self-contained content"</article>
        </>
    }
}
