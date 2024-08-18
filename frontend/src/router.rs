//! frontend/src/router.rs
//! This holds the logic for defining url routes
use leptos::*;
use leptos_router::{Route, Router, Routes};

use crate::pages::*;

/// For a SPA renders a few different paths critical for users to save their favourite quizzes and
/// find users through URL patterns (when features implemented).
#[component]
pub fn AppRouter() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=HomePage/>
                <Route path="/home" view=HomePage/>
                <Route path="/new-user" view=CreateNewUser />
                // <Route path="/test" view=HomePage>
                //     <Route path=":id" view=|| view! { <p>"{id}"</p> } />
                // </Route>
                <Route path="/terms-of-service" view=TermsOfService />
                <Route path="/*any" view=|| view! { <h2>"Page not found"</h2> }/>
            </Routes>
        </Router>
    }
}
