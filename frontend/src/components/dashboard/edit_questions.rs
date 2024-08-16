//! frontend/src/components/dashboard/edit_questions.rs
//! This component will handle the initial question making procecss
use crate::{
    models::mimic_surreal::SurrealQuestionMC,
    models::questions::{EditQuestionJsonPkg, JsonQuestion, JsonQuestionMC, QuestType},
    store::AppSettings,
    utils::{Fetcher, JsonMsg},
};
use leptos::*;
use web_sys::{Headers, RequestMode, Response};

/// Calibrate a Multiple Choice question (from a mold)
/// to "calibrate" is to edit a question.
#[component]
pub fn QuestionCalibrateMC(
    quest_mc: SurrealQuestionMC,
    add_quest: Callback<QuestType>,
    pop_quest: Callback<QuestType>,
    cancel_edit: Callback<()>,
) -> impl IntoView {
    //  -- Create Signals --
    let (err_msg, set_err_msg): (ReadSignal<Option<String>>, WriteSignal<Option<String>>) =
        create_signal(None);
    let quest_sig: RwSignal<SurrealQuestionMC> = create_rw_signal(quest_mc);

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

    // -- Create Question Action for Submitting --
    let create_question = create_action(move |pkg: &String| {
        let pkg_clone = pkg.clone();
        let headers: Headers = Headers::new().unwrap();
        headers
            .set("Content-Type", "application/json;charset=UTF-8")
            .unwrap();
        let fetcher: Fetcher = Fetcher::init()
            .set_url(app_settings.backend_url.to_string() + "question-forge")
            .add_query_param("quest", &quest_sig.get().id.to_raw())
            .set_method("PUT")
            .set_headers(headers)
            .set_mode(RequestMode::Cors)
            .build();
        async move {
            let response: Response = fetcher.fetch(Some(pkg_clone)).await;
            if response.status() >= 200 && response.status() < 300 {
                let data: SurrealQuestionMC = Fetcher::response_to_struct(&response).await;
                // Pop and Add the Updated Quest
                // Sorting happens in add_quest, so should be OK
                pop_quest.call(QuestType::MC(data.clone()));
                add_quest.call(QuestType::MC(data));
            } else {
                // Displaying error if one occurs
                let deserialized: JsonMsg = Fetcher::response_to_struct(&response).await;
                set_err_msg.set(deserialized.msg.clone());
            }
        }
    });

    // -- On Submit --
    // Pull values from form and send to the action for async ingestion
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

        let pre_pkg: EditQuestionJsonPkg = EditQuestionJsonPkg {
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

    // -- View --
    view! {
        <form
            class="forge-container"
            on:submit=on_submit
        >
            <h4>"Question Calibration"</h4>
            <h4>{move || err_msg.get() }</h4>
            <input
                type="text"
                placeholder="question"
                node_ref=question_ref required
                value=move || quest_sig.get().question
            />
            <input
                type="text"
                placeholder="hint"
                node_ref=hint_ref
                value=move || quest_sig.get().hint
            />
            <input
                type="text"
                placeholder="answer"
                node_ref=answer_ref required
                value=move || quest_sig.get().answer
            />
            <input
                type="text"
                placeholder="wrong choice"
                node_ref=wrong1_ref
                value=move || quest_sig.get().choices[0].clone()
                required
            />
            <input
                type="text"
                placeholder="wrong choice"
                node_ref=wrong2_ref
                value=move || quest_sig.get().choices[1].clone()
                required
            />
            <input
                type="text"
                placeholder="wrong choice"
                node_ref=wrong3_ref
                value=move || quest_sig.get().choices[2].clone()
                required
            />
            <div
                class="button-case"
            >
                <input type="submit" value="Save" />
                <button on:click=move |_| cancel_edit.call(())>"Cancel"</button>
            </div>
        </form>
    }
}
