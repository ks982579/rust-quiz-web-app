//! frontend/src/components/dashboard/create_questions.rs
//! This component will handle the initial question making procecss
use leptos::*;

use crate::{
    components::dashboard::{QuestionMold, QuestionShowcase},
    models::mimic_surreal::SurrealQuiz,
    models::questions::{JsonQuestion, QLInternals, Quest, QuestType},
    utils::DashDisplay,
};

/// Holds Data and logic for creating and editing questions.
/// The name indicates using this component to both create and edit questions.
#[component]
pub fn QuestionForge(
    display_settings: WriteSignal<DashDisplay>,
    quiz_data: ReadSignal<Option<SurrealQuiz>>,
) -> impl IntoView {
    // -- Create Signals
    let quest_signal = create_rw_signal(Vec::<Quest>::new());
    let new_question_signal = create_rw_signal(Vec::<QLInternals>::new());
    let bin_count: RwSignal<usize> = create_rw_signal(0);
    let quest_count: RwSignal<usize> = create_rw_signal(0);

    // TODO: Make Request to get already made questions

    // -- Call backs --
    let add_question = move |_event: ev::MouseEvent| {
        new_question_signal.update(|this| {
            this.push(QLInternals {
                id: bin_count.get(),
                data: JsonQuestion::default(),
            })
        });
        bin_count.update(|val| {
            *val = *val + 1;
        });
    };
    let add_quest = Callback::new(move |q: QuestType| {
        quest_signal.update(|this| {
            this.push(Quest {
                id: quest_count.get(),
                quest: q,
            })
        });
        quest_count.update(|val| {
            *val = *val + 1;
        });
    });

    view! {
        <>
            <h1>Question Forge</h1>
            // Here would be list of already made questions
            <For
                each=move || quest_signal.get()
                key=|q| q.id.clone()
                children=move |thing| view!{
                    <QuestionShowcase
                        quest=thing
                    />
                }
            />
            <For
                each=move || new_question_signal.get()
                key=|q| q.id.clone()
                children=move |thing| view!{
                    <QuestionMold
                        question=thing
                        new_quest_rw=new_question_signal
                        quest_callback=add_quest
                        quiz_data=quiz_data
                    />
                }
            />
            <button on:click=add_question>"+ add question"</button>
        </>
    }
}
