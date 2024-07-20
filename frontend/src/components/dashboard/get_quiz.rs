//! frontend/src/components/dashboard/get_quiz.rs
//! This component will handle quiz making logic and pass
//! user to the making questions screen.
use leptos::*;
use serde::{Deserialize, Serialize};
use web_sys::{Headers, RequestMode, Response};

use crate::{
    components::Card,
    models::mimic_surreal::{SurrealQuiz, Thing},
    store::AppSettings,
    utils::Fetcher,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct QuizJsonPkg {
    pub name: String,
    pub description: String,
}

#[component]
pub fn QuizShowCase(
    quiz_list: RwSignal<Vec<SurrealQuiz>>,
    quiz_selector: Callback<SurrealQuiz>,
    pop_quiz: Callback<SurrealQuiz>,
    quiz_updater: Callback<SurrealQuiz>,
    quest_calibrate: Callback<SurrealQuiz>,
) -> impl IntoView {
    //  -- Create Signals --
    //  -- Create References --
    let quiz_title: NodeRef<html::Input> = create_node_ref();
    let quiz_description: NodeRef<html::Textarea> = create_node_ref();
    // -- Use Context --

    // -- Render View --
    view! {
        <div
            data-test="id123"
            class:quiz-showcase-container=true
        >
            <h2>"My Quizzes!"</h2>
            <For
                each=move || quiz_list.get()
                key=|q| q.id.to_raw()
                children=move |this| view! {
                    // Card has to be in the Exhibit to obtain information.
                    <QuizExhibit
                        surreal_quiz=this
                        quiz_selector=quiz_selector
                        pop_quiz=pop_quiz
                        quiz_updater=quiz_updater
                        quest_calibrate=quest_calibrate
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
    pop_quiz: Callback<SurrealQuiz>,
    quiz_updater: Callback<SurrealQuiz>,
    quest_calibrate: Callback<SurrealQuiz>,
) -> impl IntoView {
    // -- Create Signals --
    let quiz_sig: RwSignal<SurrealQuiz> = create_rw_signal(surreal_quiz);

    // -- Use Context --
    let app_settings: AppSettings =
        use_context::<AppSettings>().expect("AppSettings context not found");

    // -- Create Closures
    let take_quiz_closure = move |_| {
        quiz_selector.call(quiz_sig.get());
    };
    let update_quiz_closure = move |_| {
        quiz_updater.call(quiz_sig.get());
    };
    let calibrate_closure = move |_| {
        quest_calibrate.call(quiz_sig.get());
    };

    // -- Create Actions --
    let destroy_quiz_action = create_action(move |_| {
        let headers: Headers = Headers::new().unwrap();
        headers
            .set("Content-Type", "application/json;charset=UTF-8")
            .unwrap();
        let fetcher: Fetcher = Fetcher::init()
            .set_url(app_settings.backend_url.to_string() + "quiz-nexus")
            .add_query_param("quiz", &quiz_sig.get().id.to_raw())
            .set_method("DELETE")
            .set_headers(headers)
            .set_mode(RequestMode::Cors)
            .build();
        async move {
            let response: Response = fetcher.fetch(None).await;
            if response.status() == 200 {
                let del_quiz: SurrealQuiz = Fetcher::response_to_struct(&response).await;
                pop_quiz.call(del_quiz);
            }
        }
    });

    view! {
        <Card on_click=None>
            <h3>"Name: "{move || quiz_sig.get().name}</h3>
            <p>{move || quiz_sig.get().description}</p>
            <div
                class:horizontal-even=true
            >
                <button
                    data-note="take_quiz_button"
                    on:click=take_quiz_closure
                >"Take Quiz"</button>
                <button
                    data-note="update_quiz_button"
                    on:click=update_quiz_closure
                >"Edit"</button>
                <button data-note="calibratte_button"
                    on:click=calibrate_closure
                >"Calibrate"</button>
                <button
                    data-note="delete_quiz_button"
                    on:click=move |_| destroy_quiz_action.dispatch(())
                >"Delete Quiz"</button>
            </div>
        </Card>
    }
}
