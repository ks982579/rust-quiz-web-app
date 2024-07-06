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

/// The Id type held by Thing
/// this application will try to keep it to String
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Id {
    Number(i64),
    String(String),
}
