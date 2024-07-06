//! frontend/src/components/dashboard/create_questions.rs
//! This component will handle the initial question making procecss
use leptos::*;

use leptos_dom::logging::console_log;
// use leptos_dom::logging::console_log;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use web_sys::{Headers, RequestMode, Response};

use crate::{
    components::Card,
    models::mimic_surreal::{Id, Thing},
    store::{AppSettings, AuthState},
    utils::{DashDisplay, Fetcher, JsonMsg, PartialUser},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionJsonPkg {
    pub quiz_id: Thing,
    pub question: JsonQuestion,
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

/// Question List Internals, Used to track and
#[derive(Clone, Debug)]
pub struct QLInternals {
    pub id: usize,
    pub data: JsonQuestion,
}

/// The Mold is a generic placeholder for all question to be Cast.
#[component]
pub fn QuestionMold(id: usize, rw: RwSignal<Vec<QLInternals>>) -> impl IntoView {
    view! {
        <div>
            <p>"Only Multiple Choice at the moment"</p>
            {move || {
                match rw.get()[id].data {
                    JsonQuestion::MultipleChoice(_) => view! {
                        <QuestionCastMC
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

/// Casting a Multiple Choice question (from a mold)
#[component]
pub fn QuestionCastMC(id: usize, rw: RwSignal<Vec<QLInternals>>) -> impl IntoView {
    //  -- Create Signals --
    let (err_msg, set_err_msg): (ReadSignal<Option<String>>, WriteSignal<Option<String>>) =
        create_signal(None);

    //  -- Create References --
    let question_ref: NodeRef<html::Input> = create_node_ref();
    let hint_ref: NodeRef<html::Input> = create_node_ref();
    let answer_ref: NodeRef<html::Input> = create_node_ref();
    let wrong1_ref: NodeRef<html::Input> = create_node_ref();
    let wrong2_ref: NodeRef<html::Input> = create_node_ref();
    let wrong3_ref: NodeRef<html::Input> = create_node_ref();

    // -- Use Context --
    let app_settings: AppSettings =
        use_context::<AppSettings>().expect("AppSettings context not found");

    // --- updates
    // let update_question = move |ev| ();

    // -- Create Question Action for Submitting --
    let create_question = create_action(move |pkg: &String| {
        let pkg_clone = pkg.clone();
        let headers: Headers = Headers::new().unwrap();
        headers
            .set("Content-Type", "application/json;charset=UTF-8")
            .unwrap();
        let fetcher: Fetcher = Fetcher::init()
            .set_url(app_settings.backend_url.to_string() + "question-forge")
            .set_method("POST")
            .set_headers(headers)
            .set_mode(RequestMode::Cors)
            .build();
        async move {
            let response: Response = fetcher.fetch(Some(pkg_clone)).await;
            console_log(&response.status().to_string());
            if response.status() >= 200 && response.status() < 300 {
                // Maybe a little hacky to transform into Value, can refine in future
                let hack_data: Value = Fetcher::response_to_struct(&response).await;
                console_log(&hack_data.to_string());
                // response_setter.set(Some(hack_data));
                // display_settings.set(DashDisplay::MakeQuestions);
            } else {
                let deserialized: JsonMsg = Fetcher::response_to_struct(&response).await;
                set_err_msg.set(deserialized.msg.clone());
            }
        }
    });

    // -- On Submit --
    let on_submit = move |sub_ev: ev::SubmitEvent| {
        sub_ev.prevent_default();

        // extract values
        let question_val: String = question_ref
            .get()
            .expect("<input> should be mounted")
            .value();
        let hint_val: String = hint_ref.get().expect("<input> should be mounted").value();
        let answer_val: String = answer_ref.get().expect("<input> should be mounted").value();
        let wrong1_val: String = wrong1_ref.get().expect("<input> should be mounted").value();
        let wrong2_val: String = wrong2_ref.get().expect("<input> should be mounted").value();
        let wrong3_val: String = wrong3_ref.get().expect("<input> should be mounted").value();

        // Fixing Hint type
        let real_hint: Option<String> = if hint_val.trim() == "" {
            None
        } else {
            Some(String::from(hint_val.trim()))
        };

        // Package Data into JSON String
        let pre_pre_pkg: JsonQuestion = JsonQuestion::MultipleChoice(JsonQuestionMC {
            question: String::from(question_val.trim()),
            hint: real_hint,
            answer: answer_val.trim().to_string(),
            choices: vec![
                wrong1_val.trim().to_string(),
                wrong2_val.trim().to_string(),
                wrong3_val.trim().to_string(),
            ],
        });

        let fake_id: Id = Id::String(String::from("01907c77-1a03-7102-b963-bc610b76ca30"));
        let fake_thing: Thing = Thing {
            tb: String::from("quizzes"),
            id: fake_id,
        };

        let pre_pkg: QuestionJsonPkg = QuestionJsonPkg {
            quiz_id: fake_thing,
            question: pre_pre_pkg,
        };

        let pkg_res: serde_json::Result<String> = serde_json::to_string(&pre_pkg);
        // Gracefully handle Result
        if let Ok(pkg) = pkg_res {
            create_question.dispatch(pkg);
        } else {
            set_err_msg.set(Some(String::from("Failed to serialize data")));
        }
    };

    view! {
        <form on:submit=on_submit>
            <h4>"don't give up"</h4>
            <h4>{move || err_msg.get() }</h4>
            <input type="text" placeholder="question" node_ref=question_ref required/>
            <input type="text" placeholder="hint" node_ref=hint_ref/>
            <input type="text" placeholder="answer" node_ref=answer_ref required/>
            <input type="text" placeholder="wrong choice" node_ref=wrong1_ref required/>
            <input type="text" placeholder="wrong choice" node_ref=wrong2_ref required/>
            <input type="text" placeholder="wrong choice" node_ref=wrong3_ref required/>
            <input type="submit" value="Save" />
        </form>
    }
}
