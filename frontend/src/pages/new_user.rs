//! frontend/src/pages/new_user.rs
//! Holds component for registering a new user.
use leptos::*;
use leptos_router::A;
use leptos_router::{use_navigate, NavigateOptions};
use serde::{Deserialize, Serialize};
use std::rc::Rc;

use crate::store::AppSettings;
use crate::utils::{Fetcher, JsonMsg};

use crate::components::{CenterFormCard, Footer};
use web_sys::{Headers, RequestMode, Response};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NewUserFormData {
    name: String,
    username: String,
    password: String,
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

#[component]
pub fn CreateNewUser() -> impl IntoView {
    // Create Navigator
    let navigator = use_navigate();
    let navigator_rc = Rc::new(navigator);
    //
    // -- Use Context --
    let app_settings: AppSettings =
        use_context::<AppSettings>().expect("AppSettings context not found");

    // Signals for Error Messages
    let (err_msg, set_err_msg): (ReadSignal<Option<String>>, WriteSignal<Option<String>>) =
        create_signal(None);
    let (show_password, set_show_password) = create_signal(ShowPassword::default());

    let name_input_elm: NodeRef<html::Input> = create_node_ref();
    let username_input_elm: NodeRef<html::Input> = create_node_ref();
    let password_input_elm: NodeRef<html::Input> = create_node_ref();
    let (checked, set_checked) = create_signal(false);
    // let on_submit: dyn FnOnce = move || {};

    let attempt_signup = create_action(move |data: &String| {
        let pckg: String = data.to_owned();
        let headers: Headers = Headers::new().unwrap();
        headers
            .set("Content-Type", "application/json;charset=UTF-8")
            .unwrap();

        // Set headers for fetch
        let fetcher: Fetcher = Fetcher::init()
            .set_url(app_settings.backend_url.clone() + "create-user")
            .set_method("POST")
            .set_headers(headers)
            .set_mode(RequestMode::Cors)
            .build();

        let navigator_clone = navigator_rc.clone();

        // Fetch and receive
        async move {
            let response: Response = fetcher.fetch(Some(pckg)).await;
            if response.status() >= 200 && response.status() < 300 {
                navigator_clone("/", NavigateOptions::default());
            }

            let deserialized: JsonMsg = Fetcher::response_to_struct(&response).await;
            set_err_msg.set(deserialized.msg.clone());
        }
    });

    // Submit function for sending data to backend
    let on_submit = move |ev: ev::SubmitEvent| {
        // stop page reloading
        ev.prevent_default();
        // extract value from input
        if checked.get() {
            let name_value: String = name_input_elm
                .get() // Option<HtmlElement<T>>
                .expect("<input> should be mounted")
                // `leptos::HtmlElement<html::Input>` implements `Deref`
                .value(); // -> String
            let username_value: String = username_input_elm
                .get()
                .expect("<input> should be mounted")
                .value();
            let password_value: String = password_input_elm
                .get()
                .expect("<input> should be mounted")
                .value();

            // Package Data into JSON String
            let pckg: String = serde_json::json! ({
                "name": name_value,
                "username": username_value,
                "password": password_value,
            })
            .to_string();
            attempt_signup.dispatch(pckg);
        } else {
            set_err_msg.set(Some(String::from("Please accept the terms and conditions")));
        }
    };

    let toggle_password = move |_| match show_password.get().show {
        true => set_show_password.set(ShowPassword {
            show: false,
            input_type: "password".to_string(),
            span_class: "toggle-password".to_string(),
        }),
        false => set_show_password.set(ShowPassword {
            show: true,
            input_type: "text".to_string(),
            span_class: "toggle-password show".to_string(),
        }),
    };

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
                <h2>Sign Up</h2>
                <p><b>{move || { err_msg.get() } }</b></p>
                <form on:submit=on_submit>
                    <input type="text" id="name" node_ref=name_input_elm placeholder="Name" required/>
                    <input type="text" id="username" node_ref=username_input_elm placeholder="Username" required/>
                    <div
                        style="width: 100%"
                    >
                        <input type=move || { show_password.get().input_type } id="password" node_ref=password_input_elm placeholder="Password" required/>
                        <span id="togglePassword" class=move || {show_password.get().span_class} on:click=toggle_password>Show</span>
                    </div>
                    <div
                        style="margin-bottom: 0.5rem"
                    >
                        <label for="tos_acceptance">"I accept the terms of service for creating an account."</label>
                        <br />
                        <input
                            id="tos_acceptance"
                            type="checkbox"
                            prop:checked=checked
                            on:change=move |_| set_checked.update(|val| {
                                *val = !*val;
                            })
                            required
                        />
                        <span>
                            "I accept the "
                            <A href="/terms-of-service">"terms of service"</A>
                        </span>
                    </div>
                    <input type="submit" value="Join!" />
                </form>
                <br/>
                <A href="/home">"Back to home page"</A>
                </CenterFormCard>
            </main>
            <Footer />
        </div>
    }
}
