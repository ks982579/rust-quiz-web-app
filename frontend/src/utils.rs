//! frontend/src/utils.rs
//! Location subject to change
//! File to house helper functions that can be used across components
use leptos::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Uint8Array;
use web_sys::{wasm_bindgen::prelude::*, Headers, RequestInit, RequestMode};

// Should be a builder whose finish is a fetch that returns JSON or something.
pub struct Fetcher {
    method: String,
    headers: Headers,
    mode: RequestMode,
    url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonMsg {
    pub msg: Option<String>,
}

impl std::default::Default for JsonMsg {
    fn default() -> Self {
        Self { msg: None }
    }
}

impl Fetcher {
    pub fn init() -> FetchBuilder {
        FetchBuilder::default()
    }
    pub async fn fetch(&self, json_package: Option<String>) -> web_sys::Response {
        let mut options: RequestInit = web_sys::RequestInit::new();
        options.method(&self.method);
        options.headers(&self.headers);
        options.mode(self.mode.clone());
        options.credentials(web_sys::RequestCredentials::Include);

        if let Some(pckg) = json_package {
            options.body(Some(&JsValue::from_str(&pckg)));
        }

        let request: web_sys::Request =
            web_sys::Request::new_with_str_and_init(&self.url, &options)
                .expect("Failed to create request");

        let window: web_sys::Window = web_sys::window().expect("Failed to get Window Object");
        JsFuture::from(window.fetch_with_request(&request))
            .await
            .expect("Failed to make fetch request")
            .dyn_into()
            .expect("Failed to cast response")
    }

    pub async fn response_to_struct<T: DeserializeOwned>(response: &web_sys::Response) -> T {
        let response_body_promise = response
            .array_buffer()
            .expect("Failed to convert response to array buffer");
        let js_value: JsValue = JsFuture::from(response_body_promise)
            .await
            .expect("Failed to convert response to future");
        // This is an owned value created from the JsValue
        // allowing us to utilize the `DeserializeOwned` trait
        let uint8_array: Uint8Array = Uint8Array::new(&js_value);
        let response_body: Vec<u8> = uint8_array.to_vec();
        let deserialized: T =
            serde_json::from_slice(&response_body).expect("Failed to deserialized response");
        deserialized
    }
}

pub struct FetchBuilder {
    method: String,
    headers: Headers,
    mode: RequestMode,
    url: String,
}

/// This implements builder design pattern (I think)
impl FetchBuilder {
    pub fn set_method(mut self, method: &str) -> Self {
        self.method = method.to_owned();
        self
    }
    pub fn set_headers(mut self, headers: Headers) -> Self {
        self.headers = headers;
        self
    }
    pub fn set_mode(mut self, mode: RequestMode) -> Self {
        self.mode = mode;
        self
    }
    pub fn set_url(mut self, url: String) -> Self {
        self.url = String::from(url);
        self
    }
    pub fn build(self) -> Fetcher {
        Fetcher {
            method: self.method,
            headers: self.headers,
            mode: self.mode,
            url: self.url,
        }
    }
}

impl std::default::Default for FetchBuilder {
    fn default() -> Self {
        let headers: Headers = Headers::new().unwrap();
        headers
            .set("Content-Type", "application/json;charset=UTF-8")
            .unwrap();
        Self {
            method: String::from("POST"),
            headers: headers,
            mode: RequestMode::Cors,
            url: "http://127.0.0.1:8000/".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Clone)]
pub struct PartialUser {
    pub uuid: String,
    pub name: String,
    pub username: String,
}

#[derive(Debug, Default, Clone)]
pub enum DashDisplay {
    #[default]
    MyQuizzes,
    MakeQuizzes,
    MakeQuestions,
    TakeQuiz,
}
