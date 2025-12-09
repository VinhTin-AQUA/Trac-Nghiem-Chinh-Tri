use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub id: i64,
    pub name: String,
    pub age: i32,
}
