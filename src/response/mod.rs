mod choice;
mod message;

use serde::Deserialize;

pub use choice::Choice;
pub use message::Message;

#[derive(Debug, Deserialize)]
pub struct Response {
    id: String,
    object: String,
    created: usize,
    model: String,
    pub choices: Vec<Choice>,
}
