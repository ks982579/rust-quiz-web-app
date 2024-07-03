//! frontend/src/components/dashboard/make_quiz.rs
//! This component will handle quiz making logic and pass
//! user to the making questions screen.
use leptos::*;

use web_sys::{Headers, RequestMode, Response};

use crate::{
    components::Card,
    store::{AppSettings, AuthState},
    utils::{DashDisplay, Fetcher, JsonMsg, PartialUser},
};
// #[derive(Serialize, Deserialize, Debug)]
pub struct QuizJsonPkg {
    pub name: String,
    pub description: String,
}

#[component]
pub fn MakeQuiz() -> impl IntoView {
    //  -- Create References --
    let quiz_title: NodeRef<html::Input> = create_node_ref();
    let quiz_description: NodeRef<html::Textarea> = create_node_ref();

    // -- Create Quiz Action for Submitting --
    let create_quiz: Action<_, _> = create_action(move |pkg: String| ());
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
    };

    // -- Render View --
    view! {
        <div data-test="id123">
            <h2>"Making Quizzes"</h2>
            <form class="make-quiz-form" on:submit=on_submit>
                <input type="text" id="quiz-title" placeholder="Quiz Title" node_ref=quiz_title required/>
                <textarea id="quiz-description" placeholder="Description..." node_ref=quiz_description required />
                <input type="submit" value="Create Quiz!" />
            </form>
        </div>
    }
}
