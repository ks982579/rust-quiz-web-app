use leptos::*;
use leptos_router::{Form, Route, Router, Routes, A};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

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

#[component]
fn CreateNewUser() -> impl IntoView {
    // let (name, set_name): (ReadSignal<String>, WriteSignal<String>) =
    //     create_signal("Uncontrolled".to_string());

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
        let pck: String = serde_json::json! ({
            "name": name_value,
            "username": username_value,
            "password": password_value,
        })
        .to_string();
        let request: Request = Request::new_with_str_and_init(
            "http://localhost:8080/what",
            &RequestInit::new()
                .method("POST")
                .mode(RequestMode::Cors)
                .body(Some(&pck.into())),
        )
        .unwrap();

        request.headers().set("Accept", "application/json");

        spawn_local(async move {
            let window = web_sys::window().unwrap();
            let this: JsFuture = JsFuture::from(window.fetch_with_request(&request));
            let that: Response = this.await.unwrap().into();
        });
        // set_name.set(value);
    };
    view! {
        <form on:submit=on_submit>
            <input type="text" id="name" node_ref=name_input_elm />
            <input type="text" id="username" node_ref=username_input_elm />
            <input type="text" id="password" node_ref=password_input_elm />
            <input type="submit" value="Join!" />
        </form>
    }
}
