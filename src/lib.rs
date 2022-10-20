use std::{fs::File, io::Read};
use types::Function;

pub mod types;

pub fn get_bigquery_function_names() -> Vec<String> {
    let mut f = File::open("output/function_names.json").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let function_names: Vec<String> = serde_json::from_str(&contents).unwrap();
    return function_names;
}

pub fn get_bigquery_function_categories() -> Vec<String> {
    let mut f = File::open("output/categories.json").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let categories: Vec<String> = serde_json::from_str(&contents).unwrap();
    return categories;
}

pub fn get_bigquery_functions() -> Vec<types::Function> {
    let mut f = File::open("output/functions.json").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let functions: Vec<Function> = serde_json::from_str(&contents).unwrap();
    return functions;
}
