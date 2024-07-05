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

#[derive(Clone, Debug)]
pub struct QLInternals {
    pub id: usize,
    pub data: JsonQuestion,
}

#[component]
pub fn QuestionListMaker(id: usize, rw: RwSignal<Vec<QLInternals>>) -> impl IntoView {
    view! {
        <div>
            <p>"Only Multiple Choice at the moment"</p>
            {move || {
                match rw.get()[id].data {
                    JsonQuestion::MultipleChoice(_) => view! {
                        <QuestionMakerMC
                            id=id
                            rw=rw
                        />
                    }
            }
        }}
        </div>
    }
}

// .questions
// .push(JsonQuestion::MultipleChoice(JsonQuestionMC {
// question: String::from(
//     "In Big O notation, which of the following represents the most efficient algorithm for large inputs?",
// ),
// hint: None,
// answer: String::from("O(log(n))"),
// choices: vec![
//     String::from("O(n^2)"),
//     String::from("O(n*log(n))"),
//     String::from("O(n)"),
// ],
#[component]
pub fn QuestionMakerMC(id: usize, rw: RwSignal<Vec<QLInternals>>) -> impl IntoView {
    // Import Enum to be concise
    use JsonQuestion::*;

    // --- updates
    let update_question = move |ev| {
        rw.update(|this| {
            this[id] = if let MultipleChoice(st) = this[id].data {
                JsonQuestionMC {
                    question: event_target_value(&ev),
                    hint: st.hint,
                    answer: st.answer,
                    choices: st.choices,
                }
            }
        })
    };

    view! {
        <div>
            <h4>"don't give up"</h4>
            <input type="text" placeholder="question" on:change=update_question required/>
            <input type="text" placeholder="hint" />
            <input type="text" placeholder="answer" required/>
            <input type="text" placeholder="wrong choice" required/>
            <input type="text" placeholder="wrong choice" required/>
            <input type="text" placeholder="wrong choice" required/>
        </div>
    }
}
