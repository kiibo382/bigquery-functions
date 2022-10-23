use std::str::FromStr;
use std::{fs::File, io::Read};

pub mod types;
mod json_types;

/// Parses `output/function_names.json` and returns a vector of function_names.
pub fn get_bigquery_function_names() -> Vec<String> {
    let mut f = File::open("output/function_names.json").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let function_names: Vec<String> = serde_json::from_str(&contents).unwrap();
    return function_names;
}

/// Parses `output/functions.json` and returns a vector of categories.
pub fn get_bigquery_function_categories() -> Vec<String> {
    let mut f = File::open("output/categories.json").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let categories: Vec<String> = serde_json::from_str(&contents).unwrap();
    return categories;
}

/// Parses `output/functions.json` and returns a vector of functions.
pub fn get_bigquery_functions() -> Vec<types::Function> {
    let mut f = File::open("output/functions.json").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let functions: Vec<json_types::Function> = serde_json::from_str(&contents).unwrap();

    let converted_functions = functions
        .into_iter()
        .map(|function| {
            types::Function::new(
                function.name,
                function
                    .arguments
                    .into_iter()
                    .map(|argument| {
                        types::Argument::new(argument.name, argument.supported_argument_type)
                    })
                    .collect(),
                types::Category::from_str(&function.category)
                    .unwrap_or(types::Category::NoCategory),
                function.return_type,
                function.description,
            )
        })
        .collect();

    return converted_functions;
}
