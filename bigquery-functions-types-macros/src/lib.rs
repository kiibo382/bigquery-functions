use proc_macro::TokenStream;

/// generate enum Category from `output/categories.json`
#[proc_macro]
pub fn enum_category(_item: TokenStream) -> TokenStream {
    let contents = include_str!("output/categories.json");
    let categories: Vec<String> = serde_json::from_str(&contents).unwrap();

    let mut enum_category = String::from("#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]\npub enum Category {\n");
    for category in &categories {
        enum_category.push_str(&format!(
            "    {},\n",
            category.split(' ').collect::<Vec<&str>>()[0]
                .replace('+', "")
                .replace("-", "_")
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
    for category in &categories {
        enum_category.push_str(&format!(
            "            \"{}\" => Ok(Category::{}),\n",
            category,
            category.split(' ').collect::<Vec<&str>>()[0]
                .replace('+', "")
                .replace("-", "_")
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

    // impl Display for Category
    enum_category.push_str(
        "
impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
",
    );
    for category in &categories {
        enum_category.push_str(&format!(
            "            Category::{} => write!(f, \"{}\"),\n",
            category.split(' ').collect::<Vec<&str>>()[0]
                .replace('+', "")
                .replace("-", "_"),
            category.split(' ').collect::<Vec<&str>>()[0]
                .replace('+', "")
                .replace("-", "_")
        ));
    }
    enum_category.push_str(
        "
            Category::NoCategory => write!(f, \"NoCategory\"),
        }
    }
}
",
    );

    enum_category.parse().unwrap()
}
