//! frontend/src/pages/home.rs
//! This is main component of Homepage.
//! The `LogIn` componenet is currently part of the `HomePage`.
//! That is why it is implemented here.
//! If it becomes its own page one day, it can (and should) be moved.
use crate::pages::Dashboard;
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
                        <>
                            <div>"Loading..."</div>
                        </>
                    }
                }
                AuthStatus::Unauthenticated => {
                    view! {
                        <>
                            <summary>"Heading for details tag"</summary>
                            <details>"additional things user can open and close as needed."</details>
                            <aside>"Content aside from content, like side bar!"</aside>
                            <section>"Defines section in document"</section>
                            <section>
                                <LogIn/>
                                <A href="/new-user">"New? Create an account here"</A>
                            </section>
                            <article>"Independent, self-contained content"</article>
                        </>
                    }
                }
                AuthStatus::Authenticated => {
                    view! {
                        <>
                            <Dashboard />
                        </>
                    }
                }
            }}}
        </>
    }
}

#[derive(Clone, Debug)]
struct ShowPassword {
    show: bool,
    input_type: String,
    span_class: String,
}

impl std::default::Default for ShowPassword {
    fn default() -> Self {
        Self {
            show: false,
            input_type: "password".to_string(),
            span_class: String::from("toggle-password"),
        }
    }
}

/// The form for user login.
#[component]
fn LogIn() -> impl IntoView {
    // Pull AuthState Context
    let auth_state: AuthState = use_context::<AuthState>().expect("AuthState context not found");
    let app_settings: AppSettings =
        use_context::<AppSettings>().expect("AppSettings context not found");

    // Create signals for component
    let (err_msg, set_err_msg): (ReadSignal<Option<String>>, WriteSignal<Option<String>>) =
        create_signal(None);
    let (show_password, set_show_password) = create_signal(ShowPassword::default());
    let (checked, set_checked) = create_signal(false);

    // Create nodes for form elements
    let username_input_elm: NodeRef<html::Input> = create_node_ref();
    let password_input_elm: NodeRef<html::Input> = create_node_ref();

    let attempt_login = create_action(move |credentials: &(String, String)| {
        let (username, password) = credentials.clone();
        let pckg: String = serde_json::json!({
            "username": username,
            "password": password,
        })
        .to_string();

        let headers: Headers = Headers::new().unwrap();
        headers
            .set("Content-Type", "application/json;charset=UTF-8")
            .unwrap();
        // headers.set("Access-Control-Allow-Origin", "true").unwrap();

        let fetcher: Fetcher = Fetcher::init()
            .set_url(app_settings.backend_url.to_string() + "user-login")
            .set_method("POST")
            .set_mode(RequestMode::Cors)
            .set_headers(headers)
            .build();

        // let request: Request =
        //     Request::new_with_str_and_init("http://127.0.0.1:8000/user-login", &options).unwrap();

        // let navigator_clone = Rc::clone(&navigator_rc);

        async move {
            let response: Response = fetcher.fetch(Some(pckg)).await;

            if response.status() == 200 {
                auth_state.set_authenticated(true);
                // navigator_clone("/dashboard", NavigateOptions::default());
            } else {
                let deserialized: JsonMsg = Fetcher::response_to_struct(&response).await;

                set_err_msg.set(deserialized.msg.clone());
            }
        }
    });

    let on_submit = move |evnt: SubmitEvent| {
        evnt.prevent_default();

        if checked.get() {
            let username_value: String = username_input_elm
                .get()
                .expect("<input> should be mounted")
                .value();
            let password_value: String = password_input_elm
                .get()
                .expect("<input> should be mounted")
                .value();
            attempt_login.dispatch((username_value, password_value));
        } else {
            set_err_msg.set(Some(String::from("Please accept use of cookies")));
        }
    };

    view! {
        <header>
            <h1>"Kev's Quiz Web App"</h1>
        </header>
        <p><b>{move || { err_msg.get() } }</b></p>
        <form  on:submit=on_submit >
            <label for="username">Username:</label>
            <br/>
            <input id="username" type="text" name="username" placeholder="username" node_ref=username_input_elm required/>
            <br/>
            <label for="password">Password:</label>
            <br/>
            <input id="password" type="password" name="password" placeholder="password" node_ref=password_input_elm required/>
            <br/>
            <div>
                <p>"This website uses cookies for user login. To log in, you must accept the use of said cookies."</p>
                <label for="cookie_acceptance">"I accept the use of essential cookies for logging in."</label>
                <br />
                <input
                    id="cookie_acceptance"
                    type="checkbox"
                    prop:checked=checked
                    on:change=move |_| set_checked.update(|val| {
                        *val = !*val;
                    })
                    required
                />
            <span>I accept the cookies</span>
            </div>
            <br/>
            <input type="submit" value="Log In"/>
            <br/>
        </form>
        <footer>"&copy; 2024 Kev's Quiz Web App"</footer>
    }
}
