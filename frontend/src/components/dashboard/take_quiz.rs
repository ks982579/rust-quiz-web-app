//! frontend/src/components/dashboard/take_quiz.rs
//! This component will handle quiz making logic and pass
//! user to the making questions screen.
use crate::{
    components::Card,
    models::mimic_surreal::{SurrealQuestionMC, SurrealQuiz, Thing},
    models::questions::AllQuestions,
    store::AppSettings,
    utils::{DashDisplay, Fetcher, JsonMsg},
};
use leptos::*;
use leptos_dom::logging::console_log;
use rand::{seq::SliceRandom, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::boxed::Box;
use std::future::Future;
use std::pin::Pin;
use web_sys::{Headers, RequestMode, Response, UrlSearchParams};

/* Look at tests
* Return AllQuestions { mc: Vec<SurrealQuestionMC>}
*/

#[component]
pub fn ExamRoom(some_quiz: Option<SurrealQuiz>) -> impl IntoView {
    // -- Create Signals --
    let mcquestions: RwSignal<Vec<SurrealQuestionMC>> = create_rw_signal(Vec::new());
    // Add more signals for additional question types

    // -- Use Context --
    let app_settings: AppSettings =
        use_context::<AppSettings>().expect("AppSettings context not found");

    // -- Create Resource --
    let quizzes_resource = create_resource(
        || (), // only render once
        move |_| {
            // should be safe to unwrap
            let quiz_id: String = if let Some(quiz) = &some_quiz {
                quiz.id.to_raw()
            } else {
                // this branch should not run
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
                    let data: AllQuestions = Fetcher::response_to_struct(&response).await;
                    // response_setter.set(Some(data));
                    // display_settings.set(DashDisplay::MakeQuestions);
                    // -- Update question signals below
                    mcquestions.set(data.mc);
                } else {
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
        // if let Some(Ok(fetched_quizzes)) = quizzes_resource.get() {
        //     quiz_list.set(fetched_quizzes);
        // }
        quizzes_resource.get();
    });

    // Shuffle Questions too

    view! {
        <h2>"Quiz Name"</h2>
        <h3>"Taking an exam"</h3>
        <For
            each=move || mcquestions.get()
            key=|q| q.id.to_raw()
            children=move |this| view! {
                <MCQuestion sq=this />
            }
        />
    }
}

#[component]
pub fn MCQuestion(sq: SurrealQuestionMC) -> impl IntoView {
    let mut choices: Vec<String> = sq.choices.clone();
    // If more than 3 choices, can randomly select 3 here
    choices.push(sq.answer.clone());

    // Shuffle Choices
    let mut randrng = thread_rng();
    choices.shuffle(&mut randrng);

    // We can tuple (anw, t/f for r/w)

    view! {
        <div>
            <p>{&sq.question}</p>
            <form>
                <input type="radio" id="q1" name="question" value="interesting"/>
                <label for="q1">{&choices[0]}</label><br />
                <input type="radio" id="q2" name="question" value="interesting"/>
                <label for="q2">{&choices[1]}</label><br />
                <input type="radio" id="q3" name="question" value="interesting"/>
                <label for="q3">{&choices[2]}</label><br />
                <input type="radio" id="q4" name="question" value="interesting"/>
                <label for="q4">{&choices[3]}</label><br />
            </form>
        </div>
    }
}
