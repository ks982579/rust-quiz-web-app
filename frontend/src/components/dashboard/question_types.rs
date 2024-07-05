//! frontend/src/components/dashboard/create_questions.rs
//! This component will handle the initial question making procecss
use leptos::*;

// use leptos_dom::logging::console_log;
use serde::{Deserialize, Serialize};
// use serde_json::Value;
// use web_sys::{Headers, RequestMode, Response};
//
// use crate::{
//     components::Card,
//     store::{AppSettings, AuthState},
//     utils::{DashDisplay, Fetcher, JsonMsg, PartialUser},
// };

#[derive(Serialize, Deserialize, Debug)]
pub struct QuizJsonPkg {
    pub name: String,
    pub description: String,
}

/// To allow for the easy transporation of data
/// If adding another type, be sure to update the `JsonPkg::validate_fields()` method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum JsonQuestion {
    MultipleChoice(JsonQuestionMC),
}

impl Default for JsonQuestion {
    fn default() -> Self {
        Self::MultipleChoice(JsonQuestionMC::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct JsonQuestionMC {
    pub question: String,
    pub hint: Option<String>,
    pub answer: String,
    pub choices: Vec<String>,
}

#[component]
pub fn QuestionEditor() -> impl IntoView {
    view! {
        <div>"Question Editor"</div>
    }
}
