pub mod graph {
    // pub mod graph_items;
    // pub use graph_items::*;
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;
    use std::collections::HashMap;

    #[derive(Clone, Debug, PartialEq)]
    struct Attribute<'a> {
        attr: &'a [(&'a str, &'a str)],
    }

    // impl<'a> PartialEq<HashMap<String, String>> for &'a [(&'a str, &'a str)] {
    //     fn eq(&self, other: &HashMap<String, String>) -> bool {
    //         for (key, value) in other {
    //             if key != self[0].0 || value != self[0].1 {
    //                 return false;
    //             }
    //         }
    //         true
    //     }
    // }

    impl<'a> PartialEq<HashMap<String, String>> for Attribute<'a> {
        fn eq(&self, other: &HashMap<String, String>) -> bool {
            for (key, value) in other {
                if key != self.attr[0].0 || value != self.attr[0].1 {
                    return false;
                }
            }
            true
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Graph<'a> {
        pub nodes: Vec<Node<'a>>,
        pub edges: Vec<Edge<'a>>,
        pub attrs: &'a [(&'a str, &'a str)],
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: Default::default(),
            }
        }

        pub fn with_nodes(mut self, nodes: &'a Vec<Node<'a>>) -> Self {
            self.nodes = nodes.to_owned();
            self
        }

        pub fn with_edges(mut self, edges: &'a Vec<Edge<'a>>) -> Self {
            self.edges = edges.to_owned();
            self
        }

        pub fn with_attrs(mut self, attrs: &'a [(&'a str, &'a str)]) -> Self {
            self.attrs = attrs;
            self
        }
    }

    pub mod graph_items {
        pub mod node {
            #[derive(Clone, Debug, PartialEq)]
            pub struct Node<'a> {
                node: &'a str,
                attrs: &'a [(&'a str, &'a str)],
            }

            impl<'a> Node<'a> {
                pub fn new(node: &'a str) -> Self {
                    Node {
                        node: node,
                        attrs: Default::default(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &'a [(&'a str, &'a str)]) -> Self {
                    self.attrs = attrs;
                    self
                }
            }
        }
        pub mod edge {
            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge<'a>(&'a str, &'a str);

            impl<'a> Edge<'a> {
                pub fn new(head: &'a str, tail: &'a str) -> Self {
                    Edge(head, tail)
                }
            }
        }
    }
}
