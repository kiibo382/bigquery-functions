use std::path::Path;
use std::str::FromStr;
use std::{fs::File, io::Read};

mod json_types;
pub mod types;

/// Parses `output/function_names.json` and returns a vector of function_names.
pub fn get_bigquery_function_names() -> Vec<String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut f = File::open(path.join("output/function_names.json")).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let function_names: Vec<String> = serde_json::from_str(&contents).unwrap();
    return function_names;
}

/// Parses `output/functions.json` and returns a vector of categories.
pub fn get_bigquery_function_categories() -> Vec<String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut f = File::open(path.join("output/categories.json")).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let categories: Vec<String> = serde_json::from_str(&contents).unwrap();
    return categories;
}

/// Parses `output/functions.json` and returns a vector of functions.
pub fn get_bigquery_functions() -> Vec<types::Function> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut f = File::open(path.join("output/functions.json")).unwrap();
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

pub fn get_distinct_allowed_categories() -> [types::Category; 3] {
    [
        types::Category::Aggregate,
        types::Category::Approximate,
        types::Category::HyperLogLog,
    ]
}
