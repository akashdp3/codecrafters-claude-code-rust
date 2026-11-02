use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Message {
    role: String,
    pub content: String,
}
