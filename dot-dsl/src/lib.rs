pub mod graph {
    // pub mod graph_items;
    // pub use graph_items::*;
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;

    pub struct Graph<'a> {
        pub nodes: Vec<Node<'a>>,
        pub edges: Vec<Edge>,
        pub attrs: &'a [&'a str],
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
    }

    pub mod graph_items {
        pub mod node {
            #[derive(Clone, Debug, PartialEq)]
            pub struct Node<'a> {
                node: &'a str,
            }

            impl<'a> Node<'a> {
                pub fn new(node: &'a str) -> Self {
                    Node { node: node }
                }
            }
        }
        pub mod edge {
            pub struct Edge;
        }
    }
}
