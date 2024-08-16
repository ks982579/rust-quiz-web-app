//! frontend/src/components/dashboard/update_quiz.rs
//! This component will handle quiz update logic and redirect
//! users back to the home screen
use crate::{
    models::{mimic_surreal::SurrealQuiz, quizzes::UpdateQuizActionPkg},
    store::AppSettings,
    utils::{DashDisplay, Fetcher, JsonMsg},
};
use leptos::*;
use web_sys::{Headers, RequestMode, Response};

/// Allowing user to make updates to existing Quiz data.
#[component]
pub fn UpdateQuiz(
    display_settings: WriteSignal<DashDisplay>,
    push_quiz: Callback<SurrealQuiz>,
    pop_quiz: Callback<SurrealQuiz>,
    quiz_rw: RwSignal<Option<SurrealQuiz>>,
) -> impl IntoView {
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
    let update_quiz = create_action(move |action_pkg: &UpdateQuizActionPkg| {
        let pkg_clone = action_pkg.pkg.clone();
        let headers: Headers = Headers::new().unwrap();
        headers
            .set("Content-Type", "application/json;charset=UTF-8")
            .unwrap();
        let fetcher: Fetcher = Fetcher::init()
            .set_url(app_settings.backend_url.to_string() + "quiz-nexus")
            .add_query_param("quiz", &action_pkg.id.clone())
            .set_method("PUT")
            .set_headers(headers)
            .set_mode(RequestMode::Cors)
            .build();
        async move {
            let response: Response = fetcher.fetch(Some(pkg_clone)).await;
            if response.status() == 200 {
                let data: SurrealQuiz = Fetcher::response_to_struct(&response).await;
                pop_quiz.call(data.clone());
                push_quiz.call(data);
                // response_setter.set(Some(data));
                display_settings.set(DashDisplay::MyQuizzes);
            } else {
                let deserialized: JsonMsg = Fetcher::response_to_struct(&response).await;
                set_err_msg.set(deserialized.msg.clone());
            }
        }
    });

    // -- Closures --
    let get_quiz_name = move || {
        if let Some(qz) = quiz_rw.get() {
            Some(qz.name)
        } else {
            None
        }
    };

    let get_quiz_description = move || {
        if let Some(qz) = quiz_rw.get() {
            Some(qz.description)
        } else {
            None
        }
    };

    let get_quiz_id = move || {
        if let Some(qz) = quiz_rw.get() {
            Some(qz.id.to_raw())
        } else {
            None
        }
    };

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

        if let Some(raw_id) = get_quiz_id() {
            let action_pkg = UpdateQuizActionPkg { id: raw_id, pkg };
            update_quiz.dispatch(action_pkg);
        } else {
            set_err_msg.set(Some(String::from("Cannot find Quiz ID")))
        }
    };

    // -- Render View --
    view! {
        <div
            data-test="id123"
            class:quiz-make-container=true
        >
            <h2>"Updating Quiz"</h2>
            <h5>{move || { err_msg.get() }}</h5>
            <form class="make-quiz-form" on:submit=on_submit>
                <input
                    type="text"
                    id="quiz-title"
                    placeholder="Quiz Title"
                    node_ref=quiz_title required
                    value=get_quiz_name
                />
                <textarea
                    id="quiz-description"
                    placeholder="Description..."
                    node_ref=quiz_description required
                >{get_quiz_description}</textarea>
                <input type="submit" value="Update Quiz!" />
            </form>
        </div>
    }
}
