//! frontend/src/components/card.rs
//! This is a simple component to hold other components
use leptos::*;

#[component]
pub fn Card(children: Children, on_click: Option<Callback<ev::MouseEvent>>) -> impl IntoView {
    view! {
        <div
            style={if let Some(_) = on_click {"cursor:pointer"} else {""}}
            on:click=move |click| {
                if let Some(callback) = on_click {
                    callback.call(click);
                }
            }
            class="gen-card"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn TodoCard(children: Children, on_click: Option<Callback<ev::MouseEvent>>) -> impl IntoView {
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
