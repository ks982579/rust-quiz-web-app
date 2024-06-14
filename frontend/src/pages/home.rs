//! frontend/src/pages/home.rs
//! This is main component of Homepage.
//! The `LogIn` componenet is currently part of the `HomePage`.
//! That is why it is implemented here.
//! If it becomes its own page one day, it can (and should) be moved.
use leptos::ev::SubmitEvent;
use leptos::logging::*;
use leptos::*;
use leptos_router::{use_navigate, NavigateOptions, A};
use models::JsonMsg;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Uint8Array;
use web_sys::{wasm_bindgen::prelude::*, Headers, Request, RequestInit, RequestMode, Response};

/// Component of main home page.
#[component]
pub fn HomePage() -> impl IntoView {
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
    // Create Navigator
    let navigator = use_navigate();
    let navigator_rc = Rc::new(navigator);
    let (err_msg, set_err_msg): (ReadSignal<Option<String>>, WriteSignal<Option<String>>) =
        create_signal(None);
    let (show_password, set_show_password) = create_signal(ShowPassword::default());
    let username_input_elm: NodeRef<html::Input> = create_node_ref();
    let password_input_elm: NodeRef<html::Input> = create_node_ref();
    let (test_thing, set_test_thing): (ReadSignal<Option<String>>, WriteSignal<Option<String>>) =
        create_signal(None);

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

        // Set headers for fetch
        let mut options = RequestInit::new();
        options.method("POST");
        options.headers(&headers);
        options.body(Some(&JsValue::from_str(&pckg)));
        options.mode(RequestMode::Cors);

        let request: Request =
            Request::new_with_str_and_init("http://127.0.0.1:8000/user-login", &options).unwrap();

        let navigator_clone = Rc::clone(&navigator_rc);

        async move {
            let window = web_sys::window().unwrap();

            let response: Response = JsFuture::from(window.fetch_with_request(&request))
                .await
                .unwrap()
                .dyn_into()
                .unwrap();

            if response.status() == 200 {
                navigator_clone("/home", NavigateOptions::default());
            }

            let response_body_promise = response.array_buffer().unwrap();
            let js_value = JsFuture::from(response_body_promise).await.unwrap();
            let unit8_array: Uint8Array = Uint8Array::new(&js_value);
            let response_body = unit8_array.to_vec();
            let deserialized: JsonMsg = serde_json::from_slice(&response_body).unwrap();

            set_err_msg.set(deserialized.msg.clone());
        }
    });

    let on_submit = move |evnt: SubmitEvent| {
        evnt.prevent_default();
        let username_value: String = username_input_elm
            .get()
            .expect("<input> should be mounted")
            .value();
        let password_value: String = password_input_elm
            .get()
            .expect("<input> should be mounted")
            .value();
        set_test_thing.set(Some(username_value.clone()));
        attempt_login.dispatch((username_value, password_value));
    };

    view! {
        <p><b>{move || { err_msg.get() } }</b></p>
        <form  on:submit=on_submit >
            <label for="username">Username:</label>
            <br/>
            <input id="username" type="text" name="username" placeholder="username" node_ref=username_input_elm/>
            <br/>
            <label for="password">Password:</label>
            <br/>
            <input id="password" type="password" name="password" placeholder="password" node_ref=password_input_elm/>
            <br/>
            <input type="submit" value="Log In"/>
            <br/>
        </form>
    }
}
