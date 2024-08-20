//! frontend/src/components/card.rs
//! This is a simple component to hold other components
use leptos::*;

// TODO: Since other card types have been included, this should be renamed to something more
// specific.
/// Encapsulates children components in a 'card' to provide certain consistent styling
/// througout application.
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

/// Component to indicate that certain buttons and other components added to display
/// are not yet ready for use. It allows for application to contain unimplemented ideas
/// without leading user's to think a button is not working as intended.
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

/// For enclosing children components in a centered component.
#[component]
pub fn CenterFormCard(children: Children) -> impl IntoView {
    view! {
        <div
            class:form-card-container=true
        >
        <section
            // Overflow required for the unimplemented buttons
            style={"overflow: visible"}
            class:form-card=true
        >
            {children()}
        </section>
        </div>
    }
}
