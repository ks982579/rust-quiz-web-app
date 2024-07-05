//! frontend/src/components/card.rs
//! This is a simple component to hold other components
use crate::utils::DashDisplay;
use leptos::ev::MouseEvent;
use leptos::*;

#[component]
pub fn Card(children: Children, on_click: Option<WriteSignal<DashDisplay>>) -> impl IntoView {
    view! {
        <div
            style={if let Some(_) = on_click {"cursor:pointer"} else {""}}
            on:click=move |_| {
                if let Some(sig) = on_click {
                    // sig.set(DashDisplay::MakeQuizzes);
                    sig.set(DashDisplay::MakeQuestions);
                }
            }
            class="card"
        >
            {children()}
        </div>
    }
}
