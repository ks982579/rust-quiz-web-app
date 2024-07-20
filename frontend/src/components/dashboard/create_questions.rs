//! frontend/src/components/dashboard/create_questions.rs
//! This component will handle the initial question making procecss
use crate::{
    components::dashboard::{QuestionMold, QuestionShowcase},
    models::{
        mimic_surreal::SurrealQuiz,
        questions::{AllQuestions, JsonQuestion, QLInternals, QuestType},
    },
    store::AppSettings,
    utils::DashDisplay,
    utils::{Fetcher, JsonMsg},
};
use leptos::*;
use leptos_dom::logging::console_log;
use std::{boxed::Box, future::Future, pin::Pin};
use web_sys::{Headers, RequestMode, Response};

/// Holds Data and logic for creating and editing questions.
/// The name indicates using this component to both create and edit questions.
#[component]
pub fn QuestionForge(
    display_settings: WriteSignal<DashDisplay>,
    quiz_data: ReadSignal<Option<SurrealQuiz>>,
) -> impl IntoView {
    // -- Create Signals
    let quest_signal = create_rw_signal(Vec::<QuestType>::new());
    let new_question_signal = create_rw_signal(Vec::<QLInternals>::new());
    let bin_count: RwSignal<usize> = create_rw_signal(0);

    // -- Use Context --
    let app_settings: AppSettings =
        use_context::<AppSettings>().expect("AppSettings context not found");

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
            this.push(q);
            // TODO: With More QuestTypes, Sort by type first
            this.sort_by(|a, b| a.get_id().to_raw().cmp(&b.get_id().to_raw()));
        });
    });
    let remove_quest: Callback<QuestType> = Callback::new(move |dead_quest: QuestType| {
        quest_signal.update(|this| this.retain(|qst| qst.get_id() != dead_quest.get_id()));
    });

    // -- Create Resource --
    // This is pulled from the <ExamRoom /> Component
    let quizzes_resource = create_resource(
        || (), // only render once
        move |_| {
            // should be safe to unwrap
            let quiz_id: String = if let Some(quiz) = &quiz_data.get() {
                quiz.id.to_raw()
            } else {
                // this branch should not run
                // This is merely to match the expected output type
                return Box::pin(async { () }) as Pin<Box<dyn Future<Output = _>>>;
            };
            let headers: Headers = Headers::new().unwrap();
            headers
                .set("Content-Type", "application/json;charset=UTF-8")
                .unwrap();
            let fetcher: Fetcher = Fetcher::init()
                .set_url(app_settings.backend_url.clone() + "question-forge")
                .add_query_param("quiz", &quiz_id)
                .set_method("GET")
                .set_headers(headers)
                .set_mode(RequestMode::Cors)
                .build();
            Box::pin(async move {
                let response: Response = fetcher.fetch(None).await;
                if response.status() == 200 {
                    let mut data: AllQuestions = Fetcher::response_to_struct(&response).await;
                    // Sorting is In Order
                    data.mc.sort_by(|a, b| a.id.to_raw().cmp(&b.id.to_raw()));
                    // Must get data into correct type
                    for surreal_quest_mc in data.mc {
                        console_log(&serde_json::to_string(&surreal_quest_mc).unwrap());
                        quest_signal.update(|this| this.push(QuestType::MC(surreal_quest_mc)));
                    }
                    console_log("Request complete");
                } else {
                    // Todo: display error message somewhere for failed fetch?
                    let deserialized: JsonMsg = Fetcher::response_to_struct(&response).await;
                    // set_err_msg.set(deserialized.msg.clone());
                }
            }) as Pin<Box<dyn Future<Output = _>>>
        },
    );

    // -- Create Effect --
    // Runs code when signal changes
    // This resource is only set to run once, depends on ()
    create_effect(move |_| {
        quizzes_resource.get();
    });

    // -------------------------------------------------

    view! {
        <>
            <h1>Question Forge</h1>
            // Here would be list of already made questions
            <For
                each=move || quest_signal.get()
                key=|q| q.get_id().to_raw()
                children=move |quest_type| view!{
                    <QuestionShowcase
                        quest_type=quest_type
                        add_quest=add_quest
                        pop_quest=remove_quest
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
