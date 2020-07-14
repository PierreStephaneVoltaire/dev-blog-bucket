use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Post {
    pub id: u64,
    pub content: String,
    pub title: String,
    pub file_type: String,
}