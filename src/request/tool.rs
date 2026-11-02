use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolType {
    Function,
}

#[derive(Debug, Serialize)]
pub struct Function {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct Tool {
    #[serde(rename = "type")]
    tool_type: ToolType,
    function: Function,
}

impl Tool {
    pub fn new(name: impl Into<String>, description: impl Into<String>, parameters: Value) -> Self {
        Self {
            tool_type: ToolType::Function,
            function: Function {
                name: name.into(),
                description: description.into(),
                parameters,
            },
        }
    }
}
