//! frontend/src/models/quizzes.rs
//! Models for questions
use serde::{Deserialize, Serialize};

/// For sending and recieving quiz data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuizJsonPkg {
    pub name: String,
    pub description: String,
}

/// For sending and recieving quiz data in Action
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateQuizActionPkg {
    pub id: String,
    pub pkg: String,
}
