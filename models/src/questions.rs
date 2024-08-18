//! models/src/questions.rs
//! To hold question related structs
use crate::model_errors::ModelErrors;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use surrealize_macro::Surrealize;

/// Currently for deleting questions generically
/// All questions must have this Generic Data to be processed correctly
/// in all endpoints.
#[derive(Serialize, Deserialize, Debug, Surrealize)]
pub struct GenericQuestionData {
    pub question: String,
    pub author_id: String,
    pub parent_quiz: Thing,
}

/// To allow for the easy transporation of data
/// If adding another type, be sure to update the `JsonPkg::validate_fields()` method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum JsonQuestion {
    MultipleChoice(JsonQuestionMC),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonQuestionMC {
    pub question: String,
    pub hint: Option<String>,
    pub answer: String,
    pub choices: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum QuestionType {
    MultipleChoice(QuestionMC),
}

/// Multiple Choice question type
#[derive(Serialize, Deserialize, Debug, Surrealize, Clone)]
pub struct QuestionMC {
    pub question: String,
    pub hint: Option<String>,
    pub author_id: String,
    pub parent_quiz: Thing,
    pub answer: String,
    pub choices: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuestionJsonPkg {
    pub quiz_id: Thing,
    pub question: JsonQuestion,
}

impl QuestionJsonPkg {
    pub fn validate_fields(&self) -> Result<(), ModelErrors> {
        // The type system ensures quiz_id isn't empty
        use JsonQuestion::*;
        match &self.question {
            MultipleChoice(qmc) => {
                if qmc.question.trim().len() < 1 {
                    return Err(ModelErrors::JsonValidation(format!(
                        "Question cannot be empty",
                    )));
                } else if qmc.answer.trim().len() < 1 {
                    return Err(ModelErrors::JsonValidation(format!(
                        "Question needs valid answer",
                    )));
                } else if qmc.choices.len() < 1 {
                    return Err(ModelErrors::JsonValidation(format!(
                        "Question needs at least one additional choice",
                    )));
                }
                // Could loop through choices to ensure they are also not blank
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditQuestionJsonPkg {
    pub question: JsonQuestion,
}

impl EditQuestionJsonPkg {
    pub fn validate_fields(&self) -> Result<(), ModelErrors> {
        // The type system ensures quiz_id isn't empty
        use JsonQuestion::*;
        match &self.question {
            MultipleChoice(qmc) => {
                if qmc.question.trim().len() < 1 {
                    return Err(ModelErrors::JsonValidation(format!(
                        "Question cannot be empty",
                    )));
                } else if qmc.answer.trim().len() < 1 {
                    return Err(ModelErrors::JsonValidation(format!(
                        "Question needs valid answer",
                    )));
                } else if qmc.choices.len() < 1 {
                    return Err(ModelErrors::JsonValidation(format!(
                        "Question needs at least one additional choice",
                    )));
                }
                // Could loop through choices to ensure they are also not blank
            }
        }
        Ok(())
    }
}

/// This struct is for transporting All questions of a quiz to a frontend in
/// a standard format. You can add other fields for other lists of questions.
#[derive(Debug, Serialize, Deserialize)]
pub struct AllQuestions {
    // mc = Multiple Choice
    pub mc: Vec<SurrealQuestionMC>,
    // To Come: sa = Short Answer; la = Long Answer
}
