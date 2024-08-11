//! frontend/src/pages/home.rs
//! This is main component of Homepage, which acts as a wrapper for other pages
//! and displayes them conditionally on user auth status.
use crate::pages::{Dashboard, LogIn};
use crate::utils::{JsonMsg, PartialUser};
use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_router::A;
use web_sys::{Headers, RequestMode, Response};

use crate::store::{AppSettings, AuthState};
use crate::utils::Fetcher;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum AuthStatus {
    Loading,
    Authenticated,
    Unauthenticated,
}

#[component]
fn LoadingScreen() -> impl IntoView {
    view! { <div>"Loading..."</div>}
}

/// Component of main home page.
/// Checks if the user is logged in.
/// If yes, send to dashboard.
/// If no, render the login page
#[component]
pub fn HomePage() -> impl IntoView {
    let (auth_status, set_auth_status) = create_signal(AuthStatus::Loading);
    let auth_state: AuthState = use_context::<AuthState>().expect("AuthState context not found?");
    let app_settings: AppSettings =
        use_context::<AppSettings>().expect("AppSettings context not found");

    create_effect(move |_| {
        let headers: Headers = Headers::new().unwrap();
        headers
            .set("Content-Type", "application/json;charset=UTF-8")
            .unwrap();
        headers.set("Access-Control-Allow-Origin", "true").unwrap();

        let fetcher: Fetcher = Fetcher::init()
            .set_url(app_settings.backend_url.to_string() + "check-login")
            .set_method("GET")
            .set_headers(headers)
            .set_mode(RequestMode::Cors)
            .build();

        // Do not set state if it is the same, causes infinite loop
        if !auth_state.is_authenticated() {
            spawn_local(async move {
                let response: Response = fetcher.fetch(None).await;
                if response.status() == 200 {
                    let user: PartialUser = Fetcher::response_to_struct(&response).await;
                    // Putting PartialUser into Context
                    provide_context(user);
                    auth_state.set_authenticated(true);
                    set_auth_status.set(AuthStatus::Authenticated);
                } else {
                    set_auth_status.set(AuthStatus::Unauthenticated);
                }
            });
        } else {
            spawn_local(async move {
                let response: Response = fetcher.fetch(None).await;
                if response.status() == 200 {
                    let user: PartialUser = Fetcher::response_to_struct(&response).await;
                    // Putting PartialUser into Context
                    provide_context(user);
                    set_auth_status.set(AuthStatus::Authenticated);
                } else {
                    set_auth_status.set(AuthStatus::Unauthenticated);
                    auth_state.set_authenticated(false);
                }
            });
        }
    });

    view! {
        <>
            {move || {
                match auth_status.get() {
                    AuthStatus::Loading => {
                    view! {
                        <LoadingScreen />
                    }
                }
                AuthStatus::Unauthenticated => {
                    view! {
                        <LogIn/>
                    }
                }
                AuthStatus::Authenticated => {
                    view! {
                        <Dashboard />
                    }
                }
            }}}
        </>
    }
}
