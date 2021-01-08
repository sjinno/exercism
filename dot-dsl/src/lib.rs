// pub mod graph {
//     pub use crate::graph::graph_items::edge::Edge;
//     pub use crate::graph::graph_items::node::Node;
//     use std::collections::HashMap;

//     type Attribute = HashMap<String, String>;

//     #[derive(Clone)]
//     pub struct Graph<'a> {
//         pub nodes: Vec<Node<'a>>,
//         pub edges: Vec<Edge<'a>>,
//         pub attrs: Attribute,
//     }

//     impl<'a> Graph<'a> {
//         pub fn new() -> Self {
//             Graph {
//                 nodes: vec![],
//                 edges: vec![],
//                 attrs: Default::default(),
//             }
//         }

//         pub fn with_nodes(mut self, nodes: &'a Vec<Node>) -> Self {
//             self.nodes = nodes.to_vec();
//             self
//         }

//         pub fn with_edges(mut self, edges: &'a Vec<Edge>) -> Self {
//             self.edges = edges.to_vec();
//             self
//         }

//         pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
//             let mut hm = HashMap::<String, String>::new();
//             for attr in attrs {
//                 hm.insert(attr.0.to_string(), attr.1.to_string());
//             }
//             self.attrs = hm;
//             self
//         }

//         pub fn get_node(self, node: &'a str) -> Option<Node> {
//             for n in self.nodes.into_iter() {
//                 if n.0 == node {
//                     return Some(n);
//                 }
//             }
//             None
//         }
//     }

//     pub mod graph_items {
//         use super::*;
//         pub mod node {
//             use super::*;
//             #[derive(Clone, Debug, Default, PartialEq)]
//             pub struct Node<'a>(pub &'a str, pub Attribute);

//             impl<'a> Node<'a> {
//                 pub fn new(node: &'a str) -> Self {
//                     Node(node, Default::default())
//                 }

//                 pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
//                     let mut hm = HashMap::<String, String>::new();
//                     hm.insert(attrs[0].0.to_string(), attrs[0].1.to_string());
//                     Node(self.0, hm)
//                 }

//                 pub fn get_attr(&self, key: &str) -> Option<&str> {
//                     // let attr: &str = &*self.1.get(key).unwrap();
//                     // Some(attr.as_ref())
//                     self.1.get(key).map(|s| s.as_str())
//                 }
//             }
//         }
//         pub mod edge {
//             use super::*;

//             #[derive(Clone, Debug, Default, PartialEq)]
//             pub struct Edge<'a>((&'a str, &'a str), Attribute);

//             impl<'a> Edge<'a> {
//                 pub fn new(head: &'a str, tail: &'a str) -> Self {
//                     Edge((head, tail), Default::default())
//                 }

//                 pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
//                     let mut hm = HashMap::<String, String>::new();
//                     hm.insert(attrs[0].0.to_string(), attrs[0].1.to_string());
//                     Edge(self.0, hm)
//                 }
//             }
//         }
//     }
// }

// // Community solution 1
// use std::collections::HashMap;

// macro_rules! impl_attrs {
//     () => {
//         pub fn get_attr(&self, key: &str) -> Option<&str> {
//             self.attrs.get(key).map(|s| s.as_str())
//         }

//         pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
//             self.attrs = attrs
//                 .iter()
//                 .map(|(a, b)| (a.to_string(), b.to_string()))
//                 .collect();
//             self
//         }
//     };
// }

// #[derive(PartialEq, Eq, Default)]
// pub struct Graph {
//     pub nodes: Vec<Node>,
//     pub edges: Vec<Edge>,
//     pub attrs: HashMap<String, String>,
// }

// impl Graph {
//     pub fn new() -> Self {
//         Self::default()
//     }

//     pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
//         self.nodes = nodes.to_vec();
//         self
//     }

//     pub fn with_edges(mut self, edges: &[Edge]) -> Self {
//         self.edges = edges.to_vec();
//         self
//     }

//     pub fn get_node(&self, name: &str) -> Option<&Node> {
//         self.nodes.iter().find(|node| node.name == name)
//     }

//     impl_attrs!();
// }

// #[derive(PartialEq, Eq, Default, Clone, Debug)]
// pub struct Edge {
//     pub from: String,
//     pub to: String,
//     pub attrs: HashMap<String, String>,
// }

// impl Edge {
//     pub fn new(from: &str, to: &str) -> Self {
//         Edge {
//             from: from.to_string(),
//             to: to.to_string(),
//             ..Self::default()
//         }
//     }

//     impl_attrs!();
// }

// #[derive(PartialEq, Eq, Default, Clone, Debug)]
// pub struct Node {
//     pub name: String,
//     pub attrs: HashMap<String, String>,
// }

// impl Node {
//     pub fn new(name: &str) -> Self {
//         Node {
//             name: name.to_string(),
//             ..Self::default()
//         }
//     }

//     impl_attrs!();
// }

// pub mod graph {
//     pub use super::Graph;
//     pub mod graph_items {
//         pub mod edge {
//             pub use super::super::super::Edge;
//         }
//         pub mod node {
//             pub use super::super::super::Node;
//         }
//     }
// }

// Community solution 2
#[macro_use]
extern crate maplit;

pub mod graph {
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: hashmap![],
            }
        }

        pub fn with_nodes(self, nodes: &[Node]) -> Self {
            Graph {
                nodes: Vec::from(nodes),
                ..self
            }
        }

        pub fn with_edges(self, edges: &[Edge]) -> Self {
            Graph {
                edges: Vec::from(edges),
                ..self
            }
        }

        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            Graph {
                attrs: attrs
                    .iter()
                    .map(|(n, v)| (String::from(*n), String::from(*v)))
                    .collect(),
                ..self
            }
        }

        pub fn get_node(&self, name: &str) -> Option<Node> {
            self.nodes.iter().find(|n| n.get_name() == name).cloned()
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(PartialEq, Eq, Clone, Debug)]
            pub struct Edge {
                start: String,
                end: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(start: &str, end: &str) -> Edge {
                    Edge {
                        start: String::from(start),
                        end: String::from(end),
                        attrs: hashmap![],
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    Edge {
                        attrs: attrs
                            .iter()
                            .map(|(n, v)| (String::from(*n), String::from(*v)))
                            .collect(),
                        ..self
                    }
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(PartialEq, Eq, Clone, Debug)]
            pub struct Node {
                name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Node {
                    Node {
                        name: String::from(name),
                        attrs: hashmap![],
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    Node {
                        attrs: attrs
                            .iter()
                            .map(|(n, v)| (String::from(*n), String::from(*v)))
                            .collect(),
                        ..self
                    }
                }

                pub fn get_name(&self) -> &str {
                    &self.name
                }

                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(|s| s.as_str())
                }
            }
        }
    }
}
