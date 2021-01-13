macro_rules! attributes {
    ($cls:ident, $attrs:expr) => {
        Self {
            attrs: $attrs
                .iter()
                .map(|attr| {
                    let (key, val) = attr;
                    (key.to_string(), val.to_string())
                })
                .collect(),
            ..$cls
        }
    };
}

pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: vec![],
                edges: vec![],
                attrs: Default::default(),
            }
        }

        pub fn with_nodes(self, nodes: &[Node]) -> Self {
            Self {
                nodes: nodes.to_vec(),
                ..self
            }
        }

        pub fn with_edges(self, edges: &[Edge]) -> Self {
            Self {
                edges: edges.to_vec(),
                ..self
            }
        }

        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            attributes!(self, attrs)
        }

        pub fn get_node(self, name: &str) -> Option<Node> {
            self.nodes.iter().find(|n| n.name == name).cloned()
        }
    }

    pub mod graph_items {
        use super::*;

        pub mod edge {
            use super::*;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Self {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: Default::default(),
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    attributes!(self, attrs)
                }
            }
        }

        pub mod node {
            use super::*;
            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                        attrs: Default::default(),
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    attributes!(self, attrs)
                }

                pub fn get_attr(&self, k: &str) -> Option<&str> {
                    self.attrs.get(k).map(|val| val.as_str())
                }
            }
        }
    }
}
