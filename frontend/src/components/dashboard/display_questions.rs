//! frontend/src/components/dashboard/display_questions.rs
//! This component will handle the question rendering procecss for viewing and editing Questions
use crate::{
    components::dashboard::QuestionCalibrateMC,
    models::mimic_surreal::SurrealQuestionMC,
    models::questions::{Quest, QuestType},
    store::AppSettings,
    utils::Fetcher,
};
use leptos::*;
use web_sys::{Headers, RequestMode, Response};

/// A Dummy Component that should never be rendered
#[component]
fn Unimplemented() -> impl IntoView {
    view! {<div>"Not Implemented"</div>}
}

/// Component to display questions, for review or to be edited.
/// This component should not perform the editing.
#[component]
pub fn QuestionShowcase(
    quest_type: QuestType,
    add_quest: Callback<QuestType>,
    pop_quest: Callback<QuestType>,
) -> impl IntoView {
    // -- Create Signals
    let edit_sig = create_rw_signal(false);

    // -- Call backs --
    let choose_edit: Callback<()> = Callback::new(move |_| {
        edit_sig.set(true);
    });
    let unchoose_edit: Callback<()> = Callback::new(move |_| {
        edit_sig.set(false);
    });

    view! {
        {move || {
            if edit_sig.get() {
                match &quest_type {
                    QuestType::MC(data) => {
                        view! {
                            <QuestionCalibrateMC
                                quest_mc=data.to_owned()
                                add_quest=add_quest
                                pop_quest=pop_quest
                                cancel_edit=unchoose_edit
                            />
                        }
                    }
                    _ => view! {<Unimplemented />},
                }
            } else {
                match &quest_type {
                    QuestType::MC(data) => {
                        view! {
                            <QuestionExhibitMC
                                data=data.to_owned()
                                click_edit=choose_edit
                                pop_quest=pop_quest
                            />
                        }
                    }
                    _ => view! {<Unimplemented />},
                }
            }
        }}
    }
}

#[component]
pub fn QuestionExhibitMC(
    data: SurrealQuestionMC,
    click_edit: Callback<()>,
    pop_quest: Callback<QuestType>,
) -> impl IntoView {
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
            <button
                data-note="edit_quest_button"
                on:click=move |_| click_edit.call(())
            >"Edit"</button>
            <button
                data-note="delete_quest_button"
                on:click=move |_| destroy_quest_action.dispatch(())
            >"Delete"</button>
        </div>
    }
}
