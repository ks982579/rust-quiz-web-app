//! frontend/src/components/dashboard/create_questions.rs
//! This component will handle the initial question making procecss
use leptos::*;

use leptos_dom::logging::console_log;
use serde_json::Value;
use web_sys::{Headers, RequestMode, Response};

use crate::{
    components::dashboard::{JsonQuestion, QLInternals, QuestionListMaker, QuestionMakerMC},
    store::{AppSettings, AuthState},
    utils::{DashDisplay, Fetcher, JsonMsg, PartialUser},
};

#[component]
pub fn CreateQuestions(
    display_settings: WriteSignal<DashDisplay>,
    response_getter: ReadSignal<Option<Value>>,
    response_setter: WriteSignal<Option<Value>>,
) -> impl IntoView {
    // -- Create Signals
    let question_signal = create_rw_signal(Vec::<QLInternals>::new());
    let bin_count: RwSignal<usize> = create_rw_signal(0);
    if let Some(val) = response_getter.get() {
        console_log(&val.to_string());
    };
    let surreal_id: Option<Value> = response_getter.get();
    let mut fun_vec: Vec<HtmlElement<html::Div>> = Vec::new();

    // -- Call backs --
    let add_question = move |_event: ev::MouseEvent| {
        question_signal.update(|this| {
            this.push(QLInternals {
                id: bin_count.get(),
                data: JsonQuestion::default(),
            })
        });
        bin_count.update(|val| {
            *val = *val + 1;
        });
    };

    let submit_all = move |_event: ev::MouseEvent| {
        ();
    };

    view! {
        <>
            <h1>Question Forge</h1>
            <For
                each=move || question_signal.get()
                key=|q| q.id.clone()
                children=move |thing| view!{
                    <QuestionListMaker
                        id=thing.id
                        rw=question_signal
                    />
                }
            />
            <button on:click=add_question>"+ add question"</button>
            <button on:click=submit_all>"submit all questions"</button>
        </>
    }
}
