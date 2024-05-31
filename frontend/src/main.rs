use leptos::*;
use leptos_router::{Form, Route, Router, Routes, A};

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

#[component]
fn CreateNewUser() -> impl IntoView {
    let (name, set_name): (ReadSignal<String>, WriteSignal<String>) =
        create_signal("Uncontrolled".to_string());
    let input_element: NodeRef<html::Input> = create_node_ref();
    // let on_submit: dyn FnOnce = move || {};
    let on_submit = move |ev: ev::SubmitEvent| {
        // stop page reloading
        ev.prevent_default();
        // extract value from input
        let value = input_element
            .get()
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            .value();
        set_name.set(value);
    };
    view! {
        <form on:submit=on_submit>
            <input type="text" id="username" value=name node_ref=input_element />
            <input type="submit" value="Join!" />
        </form>
    }
}
