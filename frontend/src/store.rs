//! frontend/src/store.rs
//! A container for global state set into context
use leptos::*;

#[derive(Clone, Copy, Debug)]
pub struct AuthState {
    is_authenticated: RwSignal<bool>,
}

impl AuthState {
    pub fn new() -> Self {
        Self {
            is_authenticated: create_rw_signal(false),
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.is_authenticated.get()
    }

    pub fn set_authenticated(&self, authenticated: bool) {
        self.is_authenticated.set(authenticated);
    }
}
