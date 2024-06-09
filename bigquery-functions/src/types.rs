use crate::json_types;
use bigquery_functions_types_macros::enum_category;
use serde::{Deserialize, Serialize};

enum_category!();

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<Argument>,
    pub category: Category,
    pub description_markdown: String,
}

impl Function {
    pub fn new(
        name: String,
        arguments: Vec<Argument>,
        category: Category,
        description_markdown: String,
    ) -> Self {
        Function {
            name,
            arguments,
            category,
            description_markdown,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Argument {
    pub name: Option<String>,
    pub supported_argument_type: String,
}

impl Argument {
    pub fn new(name: Option<String>, supported_argument_type: String) -> Self {
        Argument {
            name,
            supported_argument_type,
        }
    }

    pub fn from_json_function_argument(json_function_argument: &json_types::Argument) -> Self {
        Self {
            name: json_function_argument.name.clone(),
            supported_argument_type: json_function_argument.supported_argument_type.clone(),
        }
    }
}
