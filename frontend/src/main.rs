use leptos::leptos_dom::logging::console_warn;
use leptos::*;
use leptos::{html::*, leptos_dom::logging::console_log};
use leptos_router::{Form, Route, Router, Routes, A};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Uint8Array;
use web_sys::{wasm_bindgen::prelude::*, Headers, Request, RequestInit, RequestMode, Response};

// Look into `reqwasm`
fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <header>
                <h1>"Kev's Quiz Web App"</h1>
                <nav>
                    <h3>"Just the Navbar section here"</h3>
                </nav>
            </header>
            <main>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/home" view=HomePage/>
                    <Route path="/new-user" view=CreateNewUser />
                    <Route path="/test" view=HomePage>
                        <Route path=":id" view=|| view! { <p>"{id}"</p> } />
                    </Route>
                    <Route path="/*any" view=|| view! { <h2>"What have you done?!?"</h2> }/>
                </Routes>
            </main>
            <footer>"&copy; 2024 Kev's Quiz Web App"</footer>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
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

#[component]
fn LogIn() -> impl IntoView {
    view! {
        <form style="border: 5px solid red;">
            <label for="username">Username:</label>
            <br/>
            <input id="username" type="text" name="username" placeholder="username"/>
            <br/>
            <label for="password">Password:</label>
            <br/>
            <input id="password" type="password" name="password" placeholder="password"/>
            <br/>
            <input type="submit" value="Log In"/>
            <br/>
        </form>
    }
}

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

impl Default for ShowPassword {
    fn default() -> Self {
        Self {
            show: false,
            input_type: "password".to_string(),
            span_class: String::from("toggle-password"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BackendJSON {
    msg: Option<String>,
}

impl Default for BackendJSON {
    fn default() -> Self {
        Self { msg: None }
    }
}

#[component]
fn CreateNewUser() -> impl IntoView {
    // let (name, set_name): (ReadSignal<String>, WriteSignal<String>) =
    //     create_signal("Uncontrolled".to_string());
    let (err_msg, set_err_msg): (ReadSignal<Option<String>>, WriteSignal<Option<String>>) =
        create_signal(None);
    let (show_password, set_show_password) = create_signal((ShowPassword::default()));

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

        // println!("{:?}", headers);
        // let this = headers.get("content-type").unwrap().unwrap();
        // web_sys::console::log_1(&JsValue::from_str(&format!("{this:?}")));

        let mut options = RequestInit::new();
        options.method("POST");
        options.headers(&headers);
        options.body(Some(&JsValue::from_str(&pckg)));
        options.mode(RequestMode::Cors);

        let request: Request =
            Request::new_with_str_and_init("http://127.0.0.1:8000/create-user", &options).unwrap();

        // request.headers().set("accept", "application/json");
        // let headers = request.headers();
        // headers.set("content-type", "application/json").unwrap();

        // let this = request.headers().get("content-type").unwrap().unwrap();
        // web_sys::console::log_1(&JsValue::from_str(&format!("{this:?}")));

        // Fetch and receive
        spawn_local(async move {
            let window = web_sys::window().unwrap();

            let response: Response = JsFuture::from(window.fetch_with_request(&request))
                .await
                .unwrap()
                .dyn_into()
                .unwrap();
            response.body();

            // convert into JSON stuck in a `JsValue`
            // let jzon: JsValue = JsFuture::from(response.json().unwrap()).await.unwrap();
            // let jzon: BackendJSON = JsFuture::from(response.json().unwrap()).await.unwrap();
            let response_body_promise = response.array_buffer().unwrap();
            let js_value = JsFuture::from(response_body_promise).await.unwrap();
            let unit8_array: Uint8Array = Uint8Array::new(&js_value);
            let response_body = unit8_array.to_vec();
            let deserialized: BackendJSON = serde_json::from_slice(&response_body).unwrap();
            // console_log(&data.msg.unwrap_or("".to_string()));
            // Literally small party as this finally works...

            // Serde implementation apparently deprecated due to dependency issues (cyclical)
            // transforming JsValue into String to massage into struct.

            // Not sure why this is an issue!!!
            // let this: String = console_warn(&jzon.as_string().unwrap());

            // let deserialized = serde_json::from_str::<BackendJSON>(&jzon.as_string().unwrap());

            // console_warn(&format!("{:?}", deserialized));
            set_err_msg.set(deserialized.msg.clone());

            // console::log_1(&jzon);
            // console::log_1(&format!("{deserialized:?}").into());
            // let json_obj = jzon.into_serde().unwrap();
            // println!("{:?}", jzon);
        })

        // spawn_local(async move {
        //     let window = web_sys::window().unwrap();
        //     let this: JsFuture = JsFuture::from(window.fetch_with_request(&request));
        //     let that: Response = this.await.unwrap().into();
        // });
        // set_name.set(value);
    };

    // let error_message: Option<HtmlElement<Div>> = (move || match err_msg.get().msg {
    //     Some(msg) => Some(view! { <div>move || { msg }</div> }),
    //     None => None,
    // })();

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
