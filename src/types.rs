use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Function {
    name: String,
    arguments: Vec<Argument>,
    category: String,
    // TODO: enum DataType
    return_type: String,
    description: String,
}

impl Function {
    pub fn new(
        name: String,
        arguments: Vec<Argument>,
        category: String,
        return_type: String,
        description: String,
    ) -> Self {
        Self {
            name,
            arguments,
            category,
            return_type,
            description,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Argument {
    name: Option<String>,
    // TODO: enum DataType
    supported_argument_type: String,
    // TODO: bool
    distinct: String,
}

impl Argument {
    pub fn new(name: Option<String>, supported_argument_type: String, distinct: String) -> Self {
        Self {
            name,
            supported_argument_type,
            distinct,
        }
    }
}
