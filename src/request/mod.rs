mod message;
mod tool;

use serde::Serialize;

use message::Message;
pub use tool::Tool;

#[derive(Debug, Serialize)]
pub struct Request {
    messages: Vec<Message>,
    model: String,
    tools: Vec<Tool>,
}

impl Request {
    pub fn new(model: String) -> Self {
        Self {
            messages: Vec::new(),
            model,
            tools: Vec::new(),
        }
    }

    pub fn with_user_message(mut self, content: String) -> Self {
        self.messages.push(Message::user(content));

        self
    }

    pub fn with_system_message(mut self, content: String) -> Self {
        self.messages.push(Message::system(content));

        self
    }

    pub fn with_tool(mut self, tool: Tool) -> Self {
        self.tools.push(tool);

        self
    }
}
