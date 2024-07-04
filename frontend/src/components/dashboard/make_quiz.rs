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
    //  -- Create Signals --
    let (err_msg, set_err_msg): (ReadSignal<Option<String>>, WriteSignal<Option<String>>) =
        create_signal(None);
    //  -- Create References --
    let quiz_title: NodeRef<html::Input> = create_node_ref();
    let quiz_description: NodeRef<html::Textarea> = create_node_ref();
    // -- Use Context --
    let app_settings: AppSettings =
        use_context::<AppSettings>().expect("AppSettings context not found");

    // -- Create Quiz Action for Submitting --
    let create_quiz = create_action(move |pkg: &String| {
        let pkg_clone = pkg.clone();
        let headers: Headers = Headers::new().unwrap();
        headers
            .set("Content-Type", "application/json;charset=UTF-8")
            .unwrap();
        let fetcher: Fetcher = Fetcher::init()
            .set_url(app_settings.backend_url.to_string() + "quiz-nexus")
            .set_method("POST")
            .set_headers(headers)
            .set_mode(RequestMode::Cors)
            .build();
        async move {
            let response: Response = fetcher.fetch(Some(pkg_clone)).await;
            if response.status() == 200 {
                let this: Option<char> = None; // change Some display to making questions
            } else {
                let deserialized: JsonMsg = Fetcher::response_to_struct(&response).await;
                set_err_msg.set(deserialized.msg.clone());
            }
        }
    });
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
        create_quiz.dispatch(pkg);
    };

    // -- Render View --
    view! {
        <div data-test="id123">
            <h2>"Making Quizzes"</h2>
            <h5>{move || { err_msg.get() }}</h5>
            <form class="make-quiz-form" on:submit=on_submit>
                <input type="text" id="quiz-title" placeholder="Quiz Title" node_ref=quiz_title required/>
                <textarea id="quiz-description" placeholder="Description..." node_ref=quiz_description required />
                <input type="submit" value="Create Quiz!" />
            </form>
        </div>
    }
}
