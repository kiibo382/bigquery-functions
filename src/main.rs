use ego_tree::NodeRef;
use scraper::{Html, Node, Selector};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("src/resources/index.html").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let fragment = Html::parse_fragment(&contents);

    // Obtain the h2 tag of id="functions" and next all the h3 tags after it
    let selector = Selector::parse("h2#functions").unwrap();
    let frag = fragment.select(&selector).next();

    let mut function_names = vec![];
    let mut functions = vec![];
    let mut categories = vec![];
    let mut category = "".to_string();

    for sib in frag.unwrap().next_siblings() {
        let node = sib.value();
        if node.is_element() {
            let elem = node.as_element().unwrap();
            if elem.name() == "h2" {
                let h2_selector = Selector::parse(&format!("h2#{}", elem.id().unwrap())).unwrap();
                let h2_frag = fragment.select(&h2_selector).next();
                category = h2_frag.unwrap().inner_html();
                categories.push(category.clone());
            }

            if elem.name() == "h3" {
                let h3_selector = Selector::parse(&format!("h3#{}", elem.id().unwrap())).unwrap();
                let h3_frag = fragment.select(&h3_selector).next();

                if h3_frag.unwrap().inner_html().contains(&" ") {
                    continue;
                }

                let mut texts = vec![];
                let mut i = 0;

                // elements after h3#function_id
                for h3_sib in h3_frag.unwrap().next_siblings() {
                    let h3_node = h3_sib.value();
                    if h3_node.is_element() {
                        let h3_elem = h3_node.as_element().unwrap();
                        if h3_elem.name() == "h3" {
                            break;
                        }
                        // Separate by strong tag
                        if h3_elem.name() == "p" {
                            if let Some(first_child) = h3_sib.first_child() {
                                if let Some(p_first_elm) = first_child.value().as_element() {
                                    if p_first_elm.name() == "strong" {
                                        i += 1;
                                        texts.push(String::new());
                                        continue;
                                    }
                                }
                            }
                        }

                        if i > 0 {
                            texts[i - 1] += &walk_node(&h3_sib);
                        }
                    }
                }

                if texts.len() > 1 {
                    function_names.push(h3_frag.unwrap().inner_html());
                    functions.push(Function::new(
                        h3_frag.unwrap().inner_html(),
                        vec![],
                        category.clone(),
                        texts[1].clone(),
                        texts[0].clone(),
                    ));
                }
            }
        }
    }

    let json = serde_json::to_string(&function_names).unwrap();
    let mut f = File::create("output/function_names.json").unwrap();
    f.write_all(json.as_bytes()).unwrap();

    let json = serde_json::to_string(&functions).unwrap();
    let mut f = File::create("output/functions.json").unwrap();
    f.write_all(json.as_bytes()).unwrap();

    let json = serde_json::to_string(&categories).unwrap();
    let mut f = File::create("output/categories.json").unwrap();
    f.write_all(json.as_bytes()).unwrap();
}

fn walk_node(node_ref: &NodeRef<Node>) -> String {
    let mut text = String::new();
    for child in node_ref.children() {
        if child.has_children() {
            text += &walk_node(&child);
        }
        let node = child.value();
        if node.is_text() {
            let node_text = node.as_text().unwrap().to_string();
            if &node_text != "\n" {
                text += &node_text;
            };
        }
        if !node.is_element() {
            text = text.trim().to_string();
            text += " ";
        }
    }
    return text.trim().to_string();
}

#[derive(Debug, Serialize, Deserialize)]
struct Function {
    name: String,
    arguments: Vec<Argument>,
    category: String,
    // TODO: enum DataType
    return_type: String,
    description: String,
}

impl Function {
    fn new(
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
struct Argument {
    name: Option<String>,
    // TODO: enum DataType
    supported_argument_type: String,
    // TODO: bool
    distinct: String,
}

impl Argument {
    fn new(name: Option<String>, supported_argument_type: String, distinct: String) -> Self {
        Self {
            name,
            supported_argument_type,
            distinct,
        }
    }
}
