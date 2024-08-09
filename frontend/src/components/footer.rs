//! frontend/src/components/footer.rs
//! This is a file for the footer component(s) for consistency.
use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <div
                class:footer-links=true
            >
                <a href="https://techhub.social/@ks982579">Mastodon</a>
                <a href="https://github.com/ks982579">Github</a>
                <a href="https://www.linkedin.com/in/kevin-sullivan-a35a4964/">LinkedIn</a>
            </div>
            <div>"© 2024 Kev's Quiz Web App"</div>
        </footer>
    }
}

#[component]
pub fn NotReal(children: Children, on_click: Option<Callback<ev::MouseEvent>>) -> impl IntoView {
    view! {
        <div
            style={if let Some(_) = on_click {"cursor:pointer"} else {""}}
            // Overflow required for the unimplemented buttons
            style={"overflow: visible"}
            on:click=move |click| {
                if let Some(callback) = on_click {
                    callback.call(click);
                }
            }
            class:gen-card=true
            class:unimplemented-feature=true
        >
            {children()}
        </div>
    }
}
