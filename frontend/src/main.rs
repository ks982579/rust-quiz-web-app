use leptos::*;

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    view! {
        <header>
            <h1>"Kev's Quiz Web App"</h1>
            <nav>
                <h3>"Just the Navbar section here"</h3>
            </nav>
        </header>
        <main>
            <summary>"Heading for details tag"</summary>
            <details>"additional things user can open and close as needed."</details>
            <aside>"Content aside from content, like side bar!"</aside>
            <section>"Defines section in document"</section>
            <section>
                <LogIn/>
            </section>
            <article>"Independent, self-contained content"</article>
        </main>
        <footer>"&copy; 2024 Kev's Quiz Web App"</footer>
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
