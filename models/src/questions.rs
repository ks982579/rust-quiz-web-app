//! models/src/questions.rs
//! To hold question related structs
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};
use surrealize_macro::Surrealize;

// pub struct QuestionType {}
pub enum QuestionType {
    MultipleChoice(QuestionMC),
}

/// Multiple Choice question type
#[derive(Serialize, Deserialize, Debug, Surrealize)]
pub struct QuestionMC {
    pub question: String,
    pub hint: Option<String>,
    pub author_id: String,
    pub parent_quiz: Thing,
    pub answer: String,
    pub choices: Vec<String>,
}
