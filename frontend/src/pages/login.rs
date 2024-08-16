//! frontend/src/pages/login.rs
//! This is the login page which is encapsulated in the Homepage.
//! Decision made to relocate file into 'components' directory as this may become its own page in
//! the future.
use crate::utils::JsonMsg;
use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_router::A;
use web_sys::{Headers, RequestMode, Response};

use crate::components::{CenterFormCard, Footer};
use crate::store::{AppSettings, AuthState};
use crate::utils::Fetcher;

// TODO: Implement the "show password" feature for logging in.
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
pub fn LogIn() -> impl IntoView {
    // Pull AuthState Context
    let auth_state: AuthState = use_context::<AuthState>().expect("AuthState context not found");
    let app_settings: AppSettings =
        use_context::<AppSettings>().expect("AppSettings context not found");

    // Create signals for component
    let (err_msg, set_err_msg): (ReadSignal<Option<String>>, WriteSignal<Option<String>>) =
        create_signal(None);
    // TODO: Implement the "show password" feature for logging in.
    let (show_password, set_show_password) = create_signal(ShowPassword::default());
    let (checked, set_checked) = create_signal(false);

    // Create nodes for form elements
    let username_input_elm: NodeRef<html::Input> = create_node_ref();
    let password_input_elm: NodeRef<html::Input> = create_node_ref();

    // -- Create action to post credentials to user login endpoint and update the user
    // authenitcation status accordingly.
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

        let fetcher: Fetcher = Fetcher::init()
            .set_url(app_settings.backend_url.to_string() + "user-login")
            .set_method("POST")
            .set_mode(RequestMode::Cors)
            .set_headers(headers)
            .build();

        async move {
            let response: Response = fetcher.fetch(Some(pckg)).await;

            if response.status() == 200 {
                auth_state.set_authenticated(true);
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

    // -- Render View --
    view! {
        <div
            class:fill-screen=true
        >
            <header>
                <h1>"Kev's Quiz App"</h1>
                <p>
                    <b>"Disclaimer"</b>
                    ": This website is a university project for educational purposes only. "
                    "Please do not enter any sensitive, personal, or confidential information into the system. "
                    "Use this site at your own risk, understanding it is a student project developed with limited time and resources."
                </p>
            </header>
            <main>
                <CenterFormCard>
                <h2>Login</h2>
                <p><b>{move || { err_msg.get() } }</b></p>
                <form  on:submit=on_submit >
                    <input id="username" type="text" name="username" placeholder="username" node_ref=username_input_elm required/>
                    <input id="password" type="password" name="password" placeholder="password" node_ref=password_input_elm required/>
                    <div
                        style="margin-bottom: 0.5rem"
                    >
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
                        <span>"I accept the cookies"</span>
                    </div>
                    <input type="submit" value="Log In"/>
                </form>
                <br/>
                <A href="/new-user">"New? Create an account here"</A>
                </CenterFormCard>
            </main>
            <Footer />
        </div>
    }
}
