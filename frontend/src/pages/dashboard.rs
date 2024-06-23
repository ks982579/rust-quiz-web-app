//! frontend/src/pages/dashboard.rs
//! This is dashboard that appears for logged in users.
use leptos::*;
use models::{JsonMsg, PartialUser};
use web_sys::{Headers, RequestMode, Response};

use crate::{store::AppSettings, Fetcher};

#[component]
fn LogoutButton() -> impl IntoView {
    let app_settings: AppSettings =
        use_context::<AppSettings>().expect("AppSettings context not found");

    let headers: Headers = Headers::new().unwrap();
    headers
        .set("Content-Type", "application/json;charset=UTF-8")
        .unwrap();
    let fetcher: Fetcher = Fetcher::init()
        .set_url(app_settings.backend_url.to_string() + "user-logout")
        .set_method("GET")
        .set_headers(headers)
        .set_mode(RequestMode::Cors)
        .build();

    let logout_action = create_action(move |_| {
        let response: Response = fetcher.fetch(None).await;
    });

    view! {
        <button
            on:click=move |_| logout_action.dispatch(())
            class="logout-button"
        >"Log Out"</button>
    }
}
/// Dashboard component to be the main logged in part of homepage.
#[component]
pub fn Dashboard() -> impl IntoView {
    let user: PartialUser = use_context().expect("PartialUser Context not set");
    view! {
        <>
            <button>"Logout"</button>
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