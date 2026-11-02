use serde::Deserialize;

use super::Message;

#[derive(Debug, Deserialize)]
pub struct Choice {
    index: usize,
    pub message: Message,
}
