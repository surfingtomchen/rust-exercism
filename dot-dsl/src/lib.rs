macro_rules! impl_attrs {
    ()=>{
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
            self
        }

        pub fn get_attr(&self, name: &str) -> Option<&str> {
            self.attrs.get(name).map(|value| value.as_str())
        }
    }
}

pub mod graph {
    use std::collections::HashMap;

    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::{Node};

    pub mod graph_items;

    #[derive(Debug, Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph::default()
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            self.nodes.extend(nodes.to_owned());
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            self.edges.extend(edges.to_owned());
            self
        }

        impl_attrs!();

        pub fn get_node(&self, label: &str) -> Option<&Node> {
            self.nodes.iter().find(|x| x.name == label)
        }
    }
}
