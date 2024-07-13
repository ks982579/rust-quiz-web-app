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
    let signal_to_grade: RwSignal<bool> = create_rw_signal(false);
    let user_grade: RwSignal<usize> = create_rw_signal(0);
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

    // Update quiz grading status
    let click_grade = move |_ev: ev::MouseEvent| {
        // Only changes to true
        if !signal_to_grade.get() {
            signal_to_grade.set(true);
        };
    };

    // -- Callback
    let score_callback: Callback<bool, ()> = Callback::new(move |correct: bool| {
        if correct {
            user_grade.update(|s| *s += 1)
        } else {
            user_grade.update(|s| *s -= 1)
        }
    });

    // Shuffle Questions too
    let shuffled_mc_questions = move || {
        let mut randrng = thread_rng();
        let mut mcqs: Vec<SurrealQuestionMC> = mcquestions.get();
        mcqs.shuffle(&mut randrng);
        return mcqs;
    };

    // -- View --
    view! {
        <h2>"Quiz Name"</h2>
        <h3>"Taking an exam"</h3>
        <For
            each=move || shuffled_mc_questions()
            key=|q| q.id.to_raw()
            children=move |this| view! {
                <MCQuestion
                    sq=this
                    to_grade=signal_to_grade
                    user_grade=score_callback
                />
            }
        />
        {
            move || {
                match signal_to_grade.get() {
                    true => None,
                    false => Some(view! { <button on:click=click_grade>"Grade Quiz"</button> } ),
                }
            }
        }
        {move || {
            if signal_to_grade.get() {
                Some(
                    view! {
                        <p>"Score: "{user_grade.get()}"/"{mcquestions.get().len()}</p>
                    }
                )
            } else {
                None
            }
        }}
    }
}

#[component]
pub fn MCQuestion(
    sq: SurrealQuestionMC,
    to_grade: RwSignal<bool>,
    user_grade: Callback<bool, ()>,
) -> impl IntoView {
    // -- Create Signals --
    let choices: RwSignal<Vec<(String, String)>> = create_rw_signal(Vec::new());
    let is_correct: RwSignal<bool> = create_rw_signal(false);
    let correct_key: RwSignal<String> = create_rw_signal(generate_random_string(16));
    let correct_answer: RwSignal<String> = create_rw_signal(sq.answer.clone());
    let radio_val: RwSignal<String> = create_rw_signal("".to_string());

    // Pairing each choicec with random value
    choices.set(
        sq.choices
            .iter()
            .map(|c| (c.clone(), generate_random_string(16)))
            .collect(),
    );
    // If more than 3 choices, can randomly select 3 here
    // --
    // Pairing the correct answer with correct_key
    choices.update(|this| this.push((sq.answer.clone(), correct_key.get().clone())));

    // Shuffle Choices
    choices.update(|this| {
        let mut randrng = thread_rng();
        this.shuffle(&mut randrng);
    });

    // We can tuple (anw, t/f for r/w)
    let radio_change = move |evnt: ev::Event| {
        let val: String = event_target_value(&evnt);
        radio_val.set(val.clone());
        if val == correct_key.get() {
            is_correct.update(|this| {
                if *this {
                    ()
                } else {
                    // this was false and is now true
                    user_grade.call(true);
                    *this = true;
                }
            })
        } else {
            is_correct.update(|this| {
                // wrong answer - if previously correct...
                if *this {
                    user_grade.call(false);
                    *this = false;
                }
            })
        }
    };

    // // -- This is a more precise `create_effect`
    // let do_grading = watch(
    //     move || to_grade.get(),
    //     move |to_grade_val, _last_grade_val_opt, _what| {
    //         user_grade.call(is_correct.get());
    //     },
    //     // run immediately?
    //     false,
    // );

    let graded_classes = move || {
        if is_correct.get() {
            "correct"
        } else {
            "incorrect"
        }
    };

    // For displaying answer
    let answer_display = move || {
        if is_correct.get() {
            None
        } else {
            let p_tag = leptos::html::p();
            p_tag.set_inner_text(&format!("Answer: {}", correct_answer.get()));
            Some(p_tag)
        }
    };

    view! {
        <div
            class:quest-case=true
            class:correct=move || is_correct.get() && to_grade.get()
            class:incorrect=move || !is_correct.get() && to_grade.get()
        >
            <p>{&sq.question}</p>
            {move || {
                if to_grade.get() {
                    view! {
                        <form>
                            <For
                                each=move || choices.get()
                                key=|c| c.1.clone()
                                children=move |this| {
                                    let is_checked = &this.1 == &radio_val.get();
                                    view! {
                                        <input
                                            type="radio"
                                            id=&this.1
                                            name="question"
                                            value=&this.1
                                            on:change=radio_change
                                            disabled=true
                                            checked=is_checked
                                        />
                                        <label for=&this.1>{&this.0}</label><br />
                                    }
                                }
                            />
                        </form>
                        {answer_display}
                    }.into_view()
                } else {
                    view! {
                        <form>
                            <For
                                each=move || choices.get()
                                key=|c| c.1.clone()
                                children=move |this| view! {
                                    <input type="radio" id=&this.1 name="question" value=&this.1 on:change=radio_change/>
                                    <label for=&this.1>{&this.0}</label><br />
                                }
                            />
                        </form>
                        <button>":)"</button>
                        <button>":("</button>
                    }.into_view()
                }
            }
        }
        </div>
    }
}

fn generate_random_string(length: usize) -> String {
    thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
