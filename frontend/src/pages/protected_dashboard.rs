//! frontend/src/pages/protected_dashboard.rs
//! This is a wrapper that checks authentication before
//! displaying components.
use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};

use crate::pages::Dashboard;
use crate::store::AuthState;

#[component]
fn CheckingAuthentication() -> impl IntoView {
    view! {
        <div>"Checking authentication..."</div>
    }
}

#[component]
pub fn ProtectedDashboard() -> impl IntoView {
    let auth_state: AuthState = use_context::<AuthState>().expect("AuthState context not found");
    let navigator = use_navigate();

    create_effect(move |_| {
        if !auth_state.is_authenticated() {
            navigator("/home", NavigateOptions::default());
        }
    });

    view! {
        {move || {
            if auth_state.is_authenticated() {
                view! { <Dashboard /> }
            } else {
                view! { <CheckingAuthentication /> }
            }
        }}
    }
}
