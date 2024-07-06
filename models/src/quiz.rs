//! models/src/quiz.rs
use crate::model_errors::ModelErrors;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use surrealize_macro::Surrealize;

#[derive(Serialize, Deserialize, Debug, Surrealize)]
pub struct Quiz {
    pub name: String,
    pub description: String,
    pub author_id: String,
    pub questions_mc: Vec<Thing>,
}

impl Quiz {
    pub fn new(name: String, description: String, author_id: String) -> Self {
        Self {
            name,
            description,
            author_id,
            questions_mc: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuizJsonPkg {
    pub name: String,
    pub description: String,
}

impl QuizJsonPkg {
    pub fn validate_field(&self) -> Result<(), ModelErrors> {
        if self.name.trim().len() < 1 {
            Err(ModelErrors::JsonValidation(String::from(
                "Quiz name cannot be blank or white space",
            )))
        } else {
            Ok(())
        }
    }
}
