use ego_tree::NodeRef;
use scraper::{Html, Node, Selector};
use serde::Serialize;
use similar::{ChangeTag, TextDiff};
use std::fs::File;
use std::io::prelude::*;

mod json_types;

fn main() {
    let resp = reqwest::blocking::get(
        "https://cloud.google.com/bigquery/docs/reference/standard-sql/functions-and-operators",
    )
    .expect("Failed to get response");

    let body = resp.text().expect("Failed to get body");

    let fragment = Html::parse_fragment(&body);

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
                        // Separate by strong
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
                    functions.push(json_types::Function::new(
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

    write_json("../output/function_names.json", &function_names);
    write_json("../output/functions.json", &functions);
    write_json("../output/categories.json", &categories);
}

fn check_diff(old: &str, new: &str) -> bool {
    TextDiff::from_lines(old, new)
        .iter_all_changes()
        .any(|change| match change.tag() {
            ChangeTag::Delete => true,
            ChangeTag::Insert => true,
            ChangeTag::Equal => false,
        })
}

fn write_json<T>(path: &str, new: T)
where
    T: Serialize,
{
    let mut contents = String::new();
    let mut f = File::open(path).unwrap();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let json = serde_json::to_string_pretty(&new).unwrap();

    if check_diff(&contents, &json) {
        println!("differ");
        let mut f = File::create(path).unwrap();
        f.write_all(json.as_bytes()).unwrap();
    }
}

fn walk_node(node_ref: &NodeRef<Node>) -> String {
    let mut text = String::new();
    for child in node_ref.children() {
        if child.has_children() {
            text += &walk_node(&child);
        }
        let node = child.value();
        if node.is_text() {
            text += &node.as_text().unwrap().to_string();
        }
    }
    return text.trim().to_string();
}
