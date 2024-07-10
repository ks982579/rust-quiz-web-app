//! frontend/src/components/dashboard/take_quiz.rs
//! This component will handle quiz making logic and pass
//! user to the making questions screen.
use crate::{
    components::Card,
    models::mimic_surreal::{SurrealQuiz, Thing},
    store::AppSettings,
    utils::{DashDisplay, Fetcher, JsonMsg},
};
use leptos::*;
use leptos_dom::logging::console_log;
use serde::{Deserialize, Serialize};
use web_sys::{Headers, RequestMode, Response};

#[component]
pub fn ExamRoom() -> impl IntoView {
    view! {
        <h3>"Taking an exam"</h3>
    }
}
