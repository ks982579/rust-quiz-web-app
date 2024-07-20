//! frontend/src/models/questions.rs
//! Models for questions
use crate::models::mimic_surreal::{SurrealQuestionMC, Thing};
use serde::{Deserialize, Serialize};

/// Struct from Models for transporting all questions for a quiz
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllQuestions {
    // mc = multiple choice
    pub mc: Vec<SurrealQuestionMC>,
}

/// Existing Questions are now Quests
/// Multiple Choice, Short Answer, Long Answer...
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum QuestType {
    MC(SurrealQuestionMC),
    LA,
    SA,
}

impl QuestType {
    pub fn get_id(&self) -> Thing {
        match &self {
            QuestType::MC(quest) => quest.id.clone(),
            _ => unimplemented!(),
        }
    }
}

/// Struct for allowing Quests to be rendered by <For />
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quest {
    pub id: usize,
    pub quest: QuestType,
}

/// For sending question, with parent Quiz information, to backend for storing in database
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuestionJsonPkg {
    pub quiz_id: Thing,
    pub question: JsonQuestion,
}

/// To allow for the easy transporation of data
/// If adding another type, be sure to update the `JsonPkg::validate_fields()` method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum JsonQuestion {
    MultipleChoice(JsonQuestionMC),
}

impl Default for JsonQuestion {
    fn default() -> Self {
        Self::MultipleChoice(JsonQuestionMC::default())
    }
}

/// Specific multiple choice question JSON format for sending information to backend
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct JsonQuestionMC {
    pub question: String,
    pub hint: Option<String>,
    pub answer: String,
    pub choices: Vec<String>,
}

/// Question List Internals, Used to track and
#[derive(Clone, Debug)]
pub struct QLInternals {
    pub id: usize,
    pub data: JsonQuestion,
}

// -- For Editing Questions --
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditQuestionJsonPkg {
    pub question: JsonQuestion,
}

// impl EditQuestionJsonPkg {
//     pub fn validate_fields(&self) -> Result<(), ModelErrors> {
//         // The type system ensures quiz_id isn't empty
//         use JsonQuestion::*;
//         match &self.question {
//             MultipleChoice(qmc) => {
//                 if qmc.question.trim().len() < 1 {
//                     return Err(ModelErrors::JsonValidation(format!(
//                         "Question cannot be empty",
//                     )));
//                 } else if qmc.answer.trim().len() < 1 {
//                     return Err(ModelErrors::JsonValidation(format!(
//                         "Question needs valid answer",
//                     )));
//                 } else if qmc.choices.len() < 1 {
//                     return Err(ModelErrors::JsonValidation(format!(
//                         "Question needs at least one additional choice",
//                     )));
//                 }
//                 // Could loop through choices to ensure they are also not blank
//             }
//         }
//         Ok(())
//     }
// }
