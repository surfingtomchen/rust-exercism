use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Node {
    pub name: String,
    attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(label: &str) -> Self {
        Node {
            name: label.to_string(),
            ..Self::default()
        }
    }

    impl_attrs!();
}