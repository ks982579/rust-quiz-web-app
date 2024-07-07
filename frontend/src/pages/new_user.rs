//! frontend/src/pages/new_user.rs
//! Holds component for registering a new user.
use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};
use serde::{Deserialize, Serialize};
use std::rc::Rc;

use crate::utils::JsonMsg;

use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Uint8Array;
use web_sys::{wasm_bindgen::prelude::*, Headers, Request, RequestInit, RequestMode, Response};

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

    // Signals for Error Messages
    let (err_msg, set_err_msg): (ReadSignal<Option<String>>, WriteSignal<Option<String>>) =
        create_signal(None);
    let (show_password, set_show_password) = create_signal(ShowPassword::default());

    let name_input_elm: NodeRef<html::Input> = create_node_ref();
    let username_input_elm: NodeRef<html::Input> = create_node_ref();
    let password_input_elm: NodeRef<html::Input> = create_node_ref();
    // let on_submit: dyn FnOnce = move || {};

    // Submit function for sending data to backend
    let on_submit = move |ev: ev::SubmitEvent| {
        // stop page reloading
        ev.prevent_default();
        // extract value from input
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
            Request::new_with_str_and_init("http://127.0.0.1:8000/create-user", &options).unwrap();
        let navigator_clone = navigator_rc.clone();

        // Fetch and receive
        spawn_local(async move {
            let window = web_sys::window().unwrap();

            let response: Response = JsFuture::from(window.fetch_with_request(&request))
                .await
                .unwrap()
                .dyn_into()
                .unwrap();

            if response.status() == 200 {
                navigator_clone("/", NavigateOptions::default());
            }

            let response_body_promise = response.array_buffer().unwrap();
            let js_value = JsFuture::from(response_body_promise).await.unwrap();
            let unit8_array: Uint8Array = Uint8Array::new(&js_value);
            let response_body = unit8_array.to_vec();
            let deserialized: JsonMsg = serde_json::from_slice(&response_body).unwrap();

            set_err_msg.set(deserialized.msg.clone());
        })
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
        <>
            <div>{ move || {  err_msg.get() }  }</div>
            <form on:submit=on_submit>
                <input type="text" id="name" node_ref=name_input_elm placeholder="Name" required/><br/>
                <input type="text" id="username" node_ref=username_input_elm placeholder="Username" required/><br/>
                <input type=move || { show_password.get().input_type } id="password" node_ref=password_input_elm placeholder="Password" required/>
                <span id="togglePassword" class=move || {show_password.get().span_class} on:click=toggle_password>Show</span><br/>
                <input type="submit" value="Join!" />
            </form>
        </>
    }
}
