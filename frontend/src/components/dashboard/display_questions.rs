//! frontend/src/components/dashboard/display_questions.rs
//! This component will handle the question rendering procecss for viewing and editing Questions
use crate::{
    models::mimic_surreal::SurrealQuestionMC,
    models::questions::{Quest, QuestType},
    store::AppSettings,
    utils::Fetcher,
};
use leptos::*;
use web_sys::{Headers, RequestMode, Response};

/// Component to display questions, for review or to be edited.
/// This component should not perform the editing.
#[component]
pub fn QuestionShowcase(quest: Quest, pop_quest: Callback<QuestType>) -> impl IntoView {
    // -- Create Signals
    // Probably need a signal to change between Exhibit and Calibrate
    let tomorrow: i32 = 0;

    // -- Call backs --
    if let 0 = tomorrow {
        match quest.quest {
            QuestType::MC(data) => view! {<QuestionExhibitMC data=data pop_quest=pop_quest/>},
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
pub fn QuestionExhibitMC(data: SurrealQuestionMC, pop_quest: Callback<QuestType>) -> impl IntoView {
    // -- Create Signals --
    let quest_signal: RwSignal<SurrealQuestionMC> = create_rw_signal(data);

    // -- Use Context --
    let app_settings: AppSettings =
        use_context::<AppSettings>().expect("AppSettings context not found");

    // -- Create Actions --
    let destroy_quest_action = create_action(move |_| {
        let headers: Headers = Headers::new().unwrap();
        headers
            .set("Content-Type", "application/json;charset=UTF-8")
            .unwrap();
        let fetcher: Fetcher = Fetcher::init()
            .set_url(app_settings.backend_url.to_string() + "question-forge")
            .add_query_param("quest", &quest_signal.get().id.to_raw())
            .set_method("DELETE")
            .set_headers(headers)
            .set_mode(RequestMode::Cors)
            .build();
        async move {
            let response: Response = fetcher.fetch(None).await;
            if response.status() == 200 {
                let del_quest: SurrealQuestionMC = Fetcher::response_to_struct(&response).await;
                pop_quest.call(QuestType::MC(del_quest));
            }
        }
    });
    view! {
        <div>
            <p>"Q: "{move || quest_signal.get().question}</p>
            <p>"Hint: "{move || quest_signal.get().hint}</p>
            <p>"A: "{move || quest_signal.get().answer}</p>
            <For
                each=move || quest_signal.get().choices.clone()
                key=|this| this.bytes().fold(0u32, |sum, byte| sum.wrapping_add(byte as u32))
                children=move |it| view! {
                    <p>"Wrong: "{it}</p>
                }
            />
            <button>"Edit"</button>
            <button
                data-note="delete_quest_button"
                on:click=move |_| destroy_quest_action.dispatch(())
            >"Delete"</button>
        </div>
    }
}
