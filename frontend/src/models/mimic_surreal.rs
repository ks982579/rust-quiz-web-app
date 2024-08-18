//! frontend/src/models/mimic_surreal.rs
//! Having trouble compiling surrealdb sdk to web assembly,
//! but this application only needs a few structs.
//! Replicating them here for compatibility.
use serde::{Deserialize, Serialize};

/// The record ID of records returned from SurrealDB.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Thing {
    pub tb: String,
    pub id: Id,
}
impl Thing {
    /// Required for passing data via query string to the backend.
    pub fn to_raw(&self) -> String {
        match &self.id {
            Id::String(it) => format!("{}:{}", self.tb, it),
            Id::Number(it) => format!("{}:{}", self.tb, it),
        }
    }
}

/// The Id type held by Thing
/// this application will try to keep it to String
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Id {
    Number(i64),
    String(String),
}

/// Multiple Choice question type
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SurrealQuestionMC {
    pub id: Thing,
    pub question: String,
    pub hint: Option<String>,
    pub author_id: String,
    pub parent_quiz: Thing,
    pub answer: String,
    pub choices: Vec<String>,
}

/// For holding Quiz data provided by backend
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SurrealQuiz {
    pub id: Thing,
    pub name: String,
    pub description: String,
    pub author_id: String,
}
