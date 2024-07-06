//! frontend/src/components/dashboard/display_questions.rs
//! This component will handle the question rendering procecss for viewing and editing Questions
use crate::{
    models::mimic_surreal::SurrealQuestionMC,
    models::questions::{Quest, QuestType},
};
use leptos::*;

/// Component to display questions, for review or to be edited.
/// This component should not perform the editing.
#[component]
pub fn QuestionShowcase(quest: Quest) -> impl IntoView {
    // -- Create Signals
    // Probably need a signal to change between Exhibit and Calibrate
    let tomorrow: i32 = 0;

    // -- Call backs --
    if let 0 = tomorrow {
        match quest.quest {
            QuestType::MC(data) => view! {<QuestionExhibitMC data=data />},
            _ => view! {<Unimplemented />},
        }
    } else {
        view! {<Unimplemented />}
    }
}

#[component]
fn Unimplemented() -> impl IntoView {
    view! {<div>"Not Implemented"</div>}
}

#[component]
pub fn QuestionExhibitMC(data: SurrealQuestionMC) -> impl IntoView {
    view! {
        <div>
            <p>"Q: "{data.question}</p>
            <p>"Hint: "{data.hint}</p>
            <p>"A: "{data.answer}</p>
            <For
                each=move || data.choices.clone()
                key=|this| this.bytes().fold(0u32, |sum, byte| sum.wrapping_add(byte as u32))
                children=move |it| view! {
                    <p>"Wrong: "{it}</p>
                }
            />
            <button>"Edit"</button>
            <button>"Delete"</button>
        </div>
    }
}
