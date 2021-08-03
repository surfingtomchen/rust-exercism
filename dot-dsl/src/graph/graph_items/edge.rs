use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Edge {
    from_node_name: String,
    to_node_name: String,
    attrs: HashMap<String, String>,
}

impl Edge {
    pub fn new(from: &str, to: &str) -> Self {
        Edge {
            from_node_name: from.to_string(),
            to_node_name: to.to_string(),
            ..Self::default()
        }
    }

    impl_attrs!();
}