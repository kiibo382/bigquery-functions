use proc_macro::TokenStream;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// generate enum Category from `output/categories.json`
#[proc_macro]
pub fn enum_category(_item: TokenStream) -> TokenStream {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let mut f = File::open(path.join("output/categories.json")).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let categories: Vec<String> = serde_json::from_str(&contents).unwrap();

    let mut enum_category = String::from("pub enum Category {\n");
    for category in categories.clone() {
        enum_category.push_str(&format!(
            "    {},\n",
            category.split(' ').collect::<Vec<&str>>()[0].replace('+', "")
        ));
    }
    enum_category.push_str("    NoCategory,\n");
    enum_category.push_str("}");

    enum_category.push_str(
        "
impl std::str::FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
",
    );
    for category in categories {
        enum_category.push_str(&format!(
            "            \"{}\" => Ok(Category::{}),\n",
            category,
            category.split(' ').collect::<Vec<&str>>()[0].replace('+', "")
        ));
    }
    enum_category.push_str(
        "
            _ => Err(()),
        }
    }
}
",
    );

    enum_category.parse().unwrap()
}
