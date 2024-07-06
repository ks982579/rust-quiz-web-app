//! frontend/src/components/dashboard/create_questions.rs
//! This component will handle the initial question making procecss
use leptos::*;

use leptos_dom::logging::console_log;
use serde_json::Value;
use web_sys::{Headers, RequestMode, Response};

use crate::{
    components::dashboard::{JsonQuestion, QLInternals, QuestionMold},
    store::{AppSettings, AuthState},
    utils::{DashDisplay, Fetcher, JsonMsg, PartialUser},
};

/// Holds Data and logic for creating and editing questions.
/// The name indicates using this component to both create and edit questions.
#[component]
pub fn QuestionForge(display_settings: WriteSignal<DashDisplay>) -> impl IntoView {
    // -- Create Signals
    let exist_question_signal = create_rw_signal(Vec::<QLInternals>::new());
    let new_question_signal = create_rw_signal(Vec::<QLInternals>::new());
    let bin_count: RwSignal<usize> = create_rw_signal(0);
    let mut fun_vec: Vec<HtmlElement<html::Form>> = Vec::new();

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

    // let submit_all = move |_event: ev::MouseEvent| {
    //     ();
    // };

    view! {
        <>
            <h1>Question Forge</h1>
            // Here would be list of already made questions
            <For
                each=move || new_question_signal.get()
                key=|q| q.id.clone()
                children=move |thing| view!{
                    <QuestionMold
                        id=thing.id
                        rw=new_question_signal
                    />
                }
            />
            <button on:click=add_question>"+ add question"</button>
            // <button on:click=submit_all>"submit all questions"</button>
        </>
    }
}
