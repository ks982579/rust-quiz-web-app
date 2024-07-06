//! models/src/model_errors.rs
//! To house errors that can occur in whatever implementations these models have.
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ModelErrors {
    JsonValidation(String),
}

impl std::fmt::Display for ModelErrors {
    fn fmt(&self, fm: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ModelErrors::*;
        match self {
            JsonValidation(msg) => write!(fm, "{}", msg),
        }
    }
}

impl std::error::Error for ModelErrors {}
