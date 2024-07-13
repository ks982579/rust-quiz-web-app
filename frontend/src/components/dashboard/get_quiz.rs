//! frontend/src/components/dashboard/get_quiz.rs
//! This component will handle quiz making logic and pass
//! user to the making questions screen.
use crate::{components::Card, models::mimic_surreal::SurrealQuiz};
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct QuizJsonPkg {
    pub name: String,
    pub description: String,
}

#[component]
pub fn QuizShowCase(
    quiz_list: RwSignal<Vec<SurrealQuiz>>,
    quiz_selector: Callback<SurrealQuiz>,
) -> impl IntoView {
    //  -- Create Signals --
    //  -- Create References --
    let quiz_title: NodeRef<html::Input> = create_node_ref();
    let quiz_description: NodeRef<html::Textarea> = create_node_ref();
    // -- Use Context --

    // -- Render View --
    view! {
        <div data-test="id123">
            <h2>"My Quizzes!"</h2>
            <For
                each=move || quiz_list.get()
                key=|q| q.id.to_raw()
                children=move |this| view! {
                    // Card has to be in the Exhibit to obtain information.
                    <QuizExhibit
                        surreal_quiz=this
                        quiz_selector=quiz_selector
                    />
                }
            />
        </div>
    }
}

#[component]
pub fn QuizExhibit(
    surreal_quiz: SurrealQuiz,
    quiz_selector: Callback<SurrealQuiz>,
) -> impl IntoView {
    let mut cloned_quiz = surreal_quiz.clone();
    let take_quiz_closure = move |_| {
        quiz_selector.call(cloned_quiz.clone());
    };
    view! {
        <Card on_click=None>
            <p>"Name: "{surreal_quiz.name}</p>
            <p>{surreal_quiz.description}</p>
            <button data-note="unimplemented" on:click=take_quiz_closure>"Take Quiz"</button>
            <button data-note="unimplemented">"Edit"</button>
            <button data-note="unimplemented">"Calibrate"</button>
            <button data-note="unimplemented">"Delete Quiz"</button>
        </Card>
    }
}
