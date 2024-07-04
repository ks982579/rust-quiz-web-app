//! frontend/src/components/dashboard/create_questions.rs
//! This component will handle the initial question making procecss
use leptos::*;

use leptos_dom::logging::console_log;
use serde_json::Value;
use web_sys::{Headers, RequestMode, Response};

use crate::{
    components::Card,
    store::{AppSettings, AuthState},
    utils::{DashDisplay, Fetcher, JsonMsg, PartialUser},
};
// #[derive(Serialize, Deserialize, Debug)]
pub struct QuizJsonPkg {
    pub name: String,
    pub description: String,
}

#[component]
pub fn CreateQuestions(
    display_settings: WriteSignal<DashDisplay>,
    response_getter: ReadSignal<Option<Value>>,
    response_setter: WriteSignal<Option<Value>>,
) -> impl IntoView {
    if let Some(val) = response_getter.get() {
        console_log(&val.to_string());
    };
    let surreal_id: Option<Value> = if let Some(val) = response_getter.get() {
        val.get("id").to_owned()
    } else {
        None
    };
    view! {
        <>
            <h1>Question Forge</h1>
        </>
    }
}
