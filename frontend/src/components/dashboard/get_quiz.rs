//! frontend/src/components/dashboard/get_quiz.rs
//! This component will handle quiz making logic and pass
//! user to the making questions screen.
use crate::{
    components::Card,
    models::mimic_surreal::{SurrealQuiz, Thing},
    store::AppSettings,
    utils::{DashDisplay, Fetcher, JsonMsg},
};
use leptos::*;
use serde::{Deserialize, Serialize};
use web_sys::{Headers, RequestMode, Response};

#[derive(Serialize, Deserialize, Debug)]
pub struct QuizJsonPkg {
    pub name: String,
    pub description: String,
}

#[component]
pub fn QuizShowCase(
    quiz_list: RwSignal<Vec<SurrealQuiz>>,
    quiz_selector: Callback<Thing>,
) -> impl IntoView {
    //  -- Create Signals --
    let (err_msg, set_err_msg): (ReadSignal<Option<String>>, WriteSignal<Option<String>>) =
        create_signal(None);
    //  -- Create References --
    let quiz_title: NodeRef<html::Input> = create_node_ref();
    let quiz_description: NodeRef<html::Textarea> = create_node_ref();
    // -- Use Context --

    // -- Create Quiz Action for Submitting --
    // let create_quiz = create_action(move |pkg: &String| {
    //     let pkg_clone = pkg.clone();
    //     let headers: Headers = Headers::new().unwrap();
    //     headers
    //         .set("Content-Type", "application/json;charset=UTF-8")
    //         .unwrap();
    //     let fetcher: Fetcher = Fetcher::init()
    //         .set_url(app_settings.backend_url.to_string() + "quiz-nexus")
    //         .set_method("POST")
    //         .set_headers(headers)
    //         .set_mode(RequestMode::Cors)
    //         .build();
    //     async move {
    //         let response: Response = fetcher.fetch(Some(pkg_clone)).await;
    //         if response.status() == 200 {
    //             let data: SurrealQuiz = Fetcher::response_to_struct(&response).await;
    //             response_setter.set(Some(data));
    //             display_settings.set(DashDisplay::MakeQuestions);
    //         } else {
    //             let deserialized: JsonMsg = Fetcher::response_to_struct(&response).await;
    //             set_err_msg.set(deserialized.msg.clone());
    //         }
    //     }
    // });

    // -- On Submit --
    let on_submit = move |subevent: ev::SubmitEvent| {
        subevent.prevent_default();
        // extract values
        let title_value: String = quiz_title.get().expect("<input> should be mounted").value();
        let description_value: String = quiz_description
            .get()
            .expect("<textarea> should be mounted")
            .value();

        // Package Data into JSON String
        let pkg: String = serde_json::json!({
            "name": title_value,
            "description": description_value
        })
        .to_string();
        // create_quiz.dispatch(pkg);
    };

    // -- Render View --
    view! {
        <div data-test="id123">
            <h2>"My Quizzes!"</h2>
            <For
                each=move || quiz_list.get()
                key=|q| q.id.to_raw()
                children=move |this| view! {
                    // Card has to be in the Exhibit to obtain information.
                    <Card on_click=None>
                        <p>"Name: "{this.name}</p>
                        <p>{this.description}</p>
                        <button data-note="unimplemented">"Take Quiz"</button>
                        <button data-note="unimplemented">"Edit"</button>
                        <button data-note="unimplemented">"Calibrate"</button>
                        <button data-note="unimplemented">"Delete Quiz"</button>
                    </Card>
                }
            />
        </div>
    }
}

pub fn QuizExhibit(surreal_quiz: SurrealQuiz, quiz_selector: Callback<Thing>) -> impl IntoView {
    let take_quiz_closure = move |_| {
        quiz_selector.call(surreal_quiz.id.clone());
    };
    view! {
        <Card on_click=None>
            <p>"Name: "{surreal_quiz.name}</p>
            <p>{surreal_quiz.description}</p>
            <button data-note="unimplemented" on:click=take_quiz_closure>"Take Quiz"</button>
            <button data-note="unimplemented">"Edit"</button>
            <button data-note="unimplemented">"Calibrate"</button>
            <button data-note="unimplemented">"Delete Quiz"</button>
        </Card>
    }
}
