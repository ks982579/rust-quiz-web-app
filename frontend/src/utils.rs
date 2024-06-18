//! frontend/src/utils.rs
//! Location subject to change
//! File to house helper functions that can be used across components
use leptos::ev::SubmitEvent;
use leptos::logging::*;
use leptos::*;
use leptos_router::{use_navigate, NavigateOptions, A};
use models::JsonMsg;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Uint8Array;
use web_sys::{wasm_bindgen::prelude::*, Headers, Request, RequestInit, RequestMode, Response};

// Should be a builder whose finish is a fetch that returns JSON or something.
pub struct Fetcher {
    method: String,
}

impl Fetcher {
    pub fn method(&mut self, method: &str) {
        self.method = method.to_owned();
    }
}
