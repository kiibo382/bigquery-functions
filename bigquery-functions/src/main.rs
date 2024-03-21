use ego_tree::NodeRef;
use indexmap::IndexMap;
use mdka::from_html;
use scraper::{Html, Node, Selector};
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use similar::{ChangeTag, TextDiff};
use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::path::Path;
use regex::Regex;

mod json_types;

struct IndexMapWrapper {
    map: IndexMap<std::string::String, Vec<std::string::String>>,
}

impl Serialize for IndexMapWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.map.len()))?;
        for (key, value) in self.map.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}

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

    let mut function_names = HashSet::new();
    let mut functions = vec![];
    let mut categories = vec![];

    for sib in frag.unwrap().next_siblings() {
        let node = sib.value();
        if node.is_element() {
            let elem = node.as_element().unwrap();
            let mut category = "".to_string();
            if elem.name() == "h2" {
                let h2_selector = Selector::parse(&format!("h2#{}", elem.id().unwrap())).unwrap();
                let h2_frag = fragment.select(&h2_selector).next();
                let re = Regex::new(r"\(.*\)").unwrap();
                category = re.replace_all(&h2_frag.unwrap().inner_html(), "").trim().to_string();
                category = category
                    .replace("functions", "")
                    .trim()
                    .replace(" ", "_")
                    .replace('+', "")
                    .replace("-", "_");
                categories.push(category.clone());
            }

            if elem.name() == "h3" {
                let h3_selector = Selector::parse(&format!("h3#{}", elem.id().unwrap())).unwrap();
                let h3_frag = fragment.select(&h3_selector).next();

                let function_name = elem.attr("data-text").unwrap().to_uppercase();

                if function_name.contains(&" ") || function_name.contains(&"(") {
                    println!("Skipping invalid function name: {}", function_name);
                    continue;
                }

                if function_names.contains(&function_name) {
                    println!("Skipping duplicate function name: {}", function_name);
                    continue;
                }

                let mut text = String::new();

                // elements after h3#function_id
                for h3_sib in h3_frag.unwrap().next_siblings() {
                    let h3_node = h3_sib.value();
                    if h3_node.is_element() {
                        let h3_elem = h3_node.as_element().unwrap();
                        if h3_elem.name() == "h3" {
                            break;
                        }
                    }
                    text += &walk_node(&h3_sib);
                }

                function_names.insert(function_name.clone());
                functions.push(json_types::Function::new(
                    function_name,
                    vec![],
                    category.clone(),
                    from_html(&text),
                ));
            }
        }
    }

    let mut function_names: Vec<_> = function_names.into_iter().collect();
    function_names.sort();
    functions.sort_by(|a, b| a.name.cmp(&b.name));
    categories.sort();

    let path = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    write_json(&path.join("output/function_names.json"), &function_names);
    write_json(&path.join("output/functions.json"), &functions);
    write_json(&path.join("output/categories.json"), &categories);
    write_json(
        &path.join("output/function_names_by_category.json"),
        IndexMapWrapper {
            map: functions.into_iter().fold(IndexMap::new(), |mut acc, f| {
                acc.entry(
                    f.category.split(' ').collect::<Vec<&str>>()[0]
                        .replace('+', "")
                        .replace("-", "_"),
                )
                .or_insert(vec![])
                .push(f.name.clone());
                acc
            }),
        },
    );
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

fn write_json<P, T>(path: P, new: T)
where
    P: AsRef<Path>,
    T: Serialize,
{
    let mut contents = String::new();
    let mut f = File::open(path.as_ref()).unwrap();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let json = serde_json::to_string_pretty(&new).unwrap();

    if check_diff(&contents, &json) {
        println!("differ");
        let mut f = File::create(path.as_ref()).unwrap();
        f.write_all(json.as_bytes()).unwrap();
    }
}

fn walk_node(node_ref: &NodeRef<Node>) -> String {
    let node = node_ref.value();
    let mut text = String::new();
    if let Some(_) = node.as_element() {
        let el = scraper::ElementRef::wrap(node_ref.clone()).unwrap();
        text += &el.html();
    } else if let Some(text_node) = node.as_text() {
        text += &text_node.trim();
    }
    return text.replace("\n", "");
}
