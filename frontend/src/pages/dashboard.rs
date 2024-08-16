//! frontend/src/pages/dashboard.rs
//! This is dashboard that appears for logged in users.
use leptos::*;
use web_sys::{Headers, RequestMode, Response};

use crate::{
    components::{
        dashboard::{ExamRoom, MakeQuiz, QuestionForge, QuizShowCase, UpdateQuiz},
        Card, Footer, TodoCard,
    },
    models::mimic_surreal::SurrealQuiz,
    store::{AppSettings, AuthState},
    utils::{DashDisplay, Fetcher, JsonMsg, PartialUser},
};

/// Component to log user out of web application
#[component]
fn LogoutButton() -> impl IntoView {
    // -- Create Context --
    let auth_state: AuthState = use_context::<AuthState>().expect("AuthState context not found?");
    let app_settings: AppSettings =
        use_context::<AppSettings>().expect("AppSettings context not found");

    // Creating user logout action to send request to logout endpoint.
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

    // -- Render View --
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

    // Resource for fetching current quizzes already create by user
    // designed to render only once, meaning other quizzes are to be added to associated vectors
    // manually. This is designed to cut down requests to server, backend, and database.
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
                    quiz_list.set(data);
                } else {
                    let _deserialized: JsonMsg = Fetcher::response_to_struct(&response).await;
                    // set_err_msg.set(deserialized.msg.clone());
                }
            }
        },
    );

    // -- Callbacks to be used throughout rest of application for quiz list management
    let add_quiz: Callback<SurrealQuiz> = Callback::new(move |new_quiz: SurrealQuiz| {
        quiz_list.update(|quizzes| {
            quizzes.push(new_quiz);
            quizzes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        });
    });
    let remove_quiz: Callback<SurrealQuiz> = Callback::new(move |dead_quiz: SurrealQuiz| {
        quiz_list.update(|q| q.retain(|qz| qz.id != dead_quiz.id));
    });

    // Create Effect to render quiz fetching resource only once when component is initialized.
    create_effect(move |_| {
        quizzes_resource.get();
    });

    // The main screen is dependent on the value of the DashDisplay Enum
    let main_screen = move || match read_display.get() {
        DashDisplay::MyQuizzes => view! {
            <QuizShowCase
                quiz_list=quiz_list
                quiz_selector=choose_quiz_to_take
                pop_quiz=remove_quiz
                quiz_updater=choose_quiz_to_update
                quest_calibrate=reforge_questions
            />
        },
        DashDisplay::MakeQuizzes => view! {
            <MakeQuiz
                display_settings=write_display
                response_setter=set_quiz_data
                push_quiz=add_quiz
            />
        },
        DashDisplay::MakeQuestions => view! {
            <QuestionForge
                display_settings=write_display
                quiz_data=quiz_data
            />
        },
        DashDisplay::TakeQuiz => view! {
            <ExamRoom some_quiz=current_quiz_rw.get()/>
        },
        DashDisplay::UpdateQuiz => view! {
            <UpdateQuiz
                display_settings=write_display
                push_quiz=add_quiz
                pop_quiz=remove_quiz
                quiz_rw=current_quiz_rw
            />
        },
    };

    // -- Render View --
    view! {
        <div
            class:fill-screen=true
        >
            <header>
                <h1>"Kev's Quiz App"</h1>
                <p>
                    <b>"Disclaimer"</b>
                    ": This website is a university project for educational purposes only. "
                    "Please do not enter any sensitive, personal, or confidential information into the system. "
                    "Use this site at your own risk, understanding it is a student project developed with limited time and resources."
                </p>
                <LogoutButton />
                // TODO: Add functional Navbar when more features are implemented
                // <nav>"left: Kev's Quiz App | Right: Find People  Notifications  Profile"</nav>
                <h2>"Welcome back "{user.name}</h2>
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
            <Footer />
        </div>
    }
}
