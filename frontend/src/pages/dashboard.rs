//! frontend/src/pages/dashboard.rs
//! This is dashboard that appears for logged in users.
use leptos::*;
use web_sys::{Headers, RequestMode, Response};

use crate::{
    components::Card,
    store::{AppSettings, AuthState},
    utils::{DashDisplay, Fetcher, JsonMsg, PartialUser},
};

#[component]
fn LogoutButton() -> impl IntoView {
    let auth_state: AuthState = use_context::<AuthState>().expect("AuthState context not found?");
    let app_settings: AppSettings =
        use_context::<AppSettings>().expect("AppSettings context not found");

    let logout_action = create_action(move |_| {
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
        async move {
            let response: Response = fetcher.fetch(None).await;
            if response.status() == 200 {
                auth_state.set_authenticated(false);
            }
        }
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
    let (read_display, write_display): (ReadSignal<DashDisplay>, WriteSignal<DashDisplay>) =
        create_signal(DashDisplay::default());

    let main_screen = move || match read_display.get() {
        DashDisplay::MyQuizzes => view! {
            <div>"GET CURRENT TESTS"</div>
            <div>"GET CURRENT TESTS"</div>
        },
        DashDisplay::MakeQuizzes => view! {
            <><div>"Making Quizzes"</div></>
        },
    };

    view! {
        <>
            <LogoutButton />
            <nav>"left: Kev's Quiz App | Right: Find People  Notifications  Profile"</nav>
            <h1>"Welcome back "{user.name}</h1>
            <div class="split-screen">
                <aside class="sidebar">
                    <Card on_click=Some(write_display)>
                        "Make a New Quiz"
                    </Card>
                    <ul>
                        <li>"Make a New Quiz"</li>
                        <li>"Saved Quizzes"</li>
                        <li>"Search Quizzes"</li>
                    </ul>
                </aside>
                <section class="main-content">
                    {main_screen}
                </section>
            </div>
        </>
    }
}
