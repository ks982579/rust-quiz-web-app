//! frontend/src/pages/dashboard.rs
//! This is dashboard that appears for logged in users.
use leptos::*;
use std::cmp::Ordering;
use web_sys::{Headers, RequestMode, Response};

use crate::{
    components::{
        dashboard::{ExamRoom, MakeQuiz, QuestionForge, QuizShowCase, UpdateQuiz},
        Card, TodoCard,
    },
    models::mimic_surreal::{SurrealQuiz, Thing},
    store::{AppSettings, AuthState},
    utils::{DashDisplay, Fetcher, JsonMsg, PartialUser},
};

/// Component to log user out of web application
#[component]
fn LogoutButton() -> impl IntoView {
    let auth_state: AuthState = use_context::<AuthState>().expect("AuthState context not found?");
    let app_settings: AppSettings =
        use_context::<AppSettings>().expect("AppSettings context not found");

    let logout_action = create_action(move |_| {
        let headers: Headers = Headers::new().unwrap();
        headers
            .set("Content-Type", "application/json;charset=UTF-8")
            .unwrap();
        let fetcher: Fetcher = Fetcher::init()
            .set_url(app_settings.backend_url.to_string() + "user-logout")
            .set_method("GET")
            .set_headers(headers)
            .set_mode(RequestMode::Cors)
            .build();
        async move {
            let response: Response = fetcher.fetch(None).await;
            if response.status() == 200 {
                auth_state.set_authenticated(false);
            }
        }
    });

    view! {
        <button
            on:click=move |_| logout_action.dispatch(())
            class="logout-button"
        >"Log Out"</button>
    }
}

/// Dashboard component to be the main logged in part of homepage.
#[component]
pub fn Dashboard() -> impl IntoView {
    // -- Create Signals --
    let (read_display, write_display): (ReadSignal<DashDisplay>, WriteSignal<DashDisplay>) =
        create_signal(DashDisplay::default());
    let current_quiz_rw: RwSignal<Option<SurrealQuiz>> = create_rw_signal(None);
    // - for holding Json data between components (like creating quiz)
    let (quiz_data, set_quiz_data): (
        ReadSignal<Option<SurrealQuiz>>,
        WriteSignal<Option<SurrealQuiz>>,
    ) = create_signal(None);
    let quiz_list = create_rw_signal(Vec::new());

    // -- Use Context --
    let user: PartialUser = use_context().expect("PartialUser Context not set");
    let app_settings: AppSettings =
        use_context::<AppSettings>().expect("AppSettings context not found");

    // -- Call backs --
    let set_display_my_quizzes = Callback::new(move |_click: ev::MouseEvent| {
        write_display.set(DashDisplay::MyQuizzes);
    });
    let set_display_make_quiz = Callback::new(move |_click: ev::MouseEvent| {
        current_quiz_rw.set(None);
        write_display.set(DashDisplay::MakeQuizzes);
    });
    // Callback to setup quiz to take
    let choose_quiz_to_take = Callback::new(move |quiz: SurrealQuiz| {
        current_quiz_rw.set(Some(quiz));
        write_display.set(DashDisplay::TakeQuiz);
    });
    let choose_quiz_to_update = Callback::new(move |qz: SurrealQuiz| {
        current_quiz_rw.set(Some(qz));
        write_display.set(DashDisplay::UpdateQuiz);
    });
    let reforge_questions = Callback::new(move |qz: SurrealQuiz| {
        set_quiz_data.set(Some(qz));
        write_display.set(DashDisplay::MakeQuestions);
    });

    // TODO: Make request for current tests
    // try `create_local_resource`
    // or `create_render_effect`
    // let settings_rc = std::rc::Rc::new(app_settings);
    let quizzes_resource = create_resource(
        || (), // only render once
        move |_| {
            let headers: Headers = Headers::new().unwrap();
            // let async_settings = std::rc::Rc::clone(&settings_rc);
            headers
                .set("Content-Type", "application/json;charset=UTF-8")
                .unwrap();
            let fetcher: Fetcher = Fetcher::init()
                .set_url(app_settings.backend_url.clone() + "quiz-nexus")
                .set_method("GET")
                .set_headers(headers)
                .set_mode(RequestMode::Cors)
                .build();
            async move {
                let response: Response = fetcher.fetch(None).await;
                if response.status() == 200 {
                    let data: Vec<SurrealQuiz> = Fetcher::response_to_struct(&response).await;
                    // response_setter.set(Some(data));
                    // display_settings.set(DashDisplay::MakeQuestions);
                    quiz_list.set(data);
                } else {
                    let deserialized: JsonMsg = Fetcher::response_to_struct(&response).await;
                    // set_err_msg.set(deserialized.msg.clone());
                }
            }
        },
    );
    let add_quiz: Callback<SurrealQuiz> = Callback::new(move |new_quiz: SurrealQuiz| {
        quiz_list.update(|quizzes| {
            quizzes.push(new_quiz);
            quizzes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        });
    });
    let remove_quiz: Callback<SurrealQuiz> = Callback::new(move |dead_quiz: SurrealQuiz| {
        quiz_list.update(|q| q.retain(|qz| qz.id != dead_quiz.id));
    });
    create_effect(move |_| {
        // if let Some(Ok(fetched_quizzes)) = quizzes_resource.get() {
        //     quiz_list.set(fetched_quizzes);
        // }
        quizzes_resource.get();
    });

    let main_screen = move || match read_display.get() {
        DashDisplay::MyQuizzes => view! {
            <>
                <QuizShowCase
                    quiz_list=quiz_list
                    quiz_selector=choose_quiz_to_take
                    pop_quiz=remove_quiz
                    quiz_updater=choose_quiz_to_update
                    quest_calibrate=reforge_questions
                />
            </>
        },
        DashDisplay::MakeQuizzes => view! {
            <><MakeQuiz display_settings=write_display response_setter=set_quiz_data/></>
        },
        DashDisplay::MakeQuestions => view! {
            <>
                <QuestionForge
                    display_settings=write_display
                    quiz_data=quiz_data
                />
            </>
        },
        DashDisplay::TakeQuiz => view! {
            <>
                <ExamRoom some_quiz=current_quiz_rw.get()/>
            </>
        },
        DashDisplay::UpdateQuiz => view! {
            <>
                <UpdateQuiz
                    display_settings=write_display
                    push_quiz=add_quiz
                    pop_quiz=remove_quiz
                    quiz_rw=current_quiz_rw
                />
            </>
        },
    };

    view! {
        <div
            class:fill-screen=true
        >
            <header>
                <LogoutButton />
                <nav>"left: Kev's Quiz App | Right: Find People  Notifications  Profile"</nav>
                <h1>"Welcome back "{user.name}</h1>
            </header>
            <main class="split-screen">
                <aside class="sidebar">
                    <Card on_click=Some(set_display_my_quizzes)>
                        "To Main Page - My Quizzes!"
                    </Card>
                    <Card on_click=Some(set_display_make_quiz)>
                        "Make a New Quiz"
                    </Card>
                    <TodoCard on_click=None>
                        "Saved Quizzes"
                    </TodoCard>
                    <TodoCard on_click=None>
                        <div
                            class:unimplemented-button=true
                        >"Search Quizzes"</div>
                    </TodoCard>
                </aside>
                <section
                    class:main-content=true
                >
                    <div
                        class:main-content-container=true
                    >
                        {main_screen}
                    </div>
                </section>
            </main>
            <footer>"&copy; 2024 Kev's Quiz Web App"</footer>
        </div>
    }
}
