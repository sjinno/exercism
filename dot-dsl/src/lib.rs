pub mod graph {
    pub use crate::graph::graph_items::edge::Edge;
    pub use crate::graph::graph_items::node::Node;
    // use std::collections::HashMap;

    impl<'a> Into<Attribute<'a>> for &'a [(&'a str, &'a str)] {
        fn into(self) -> Attribute<'a> {
            Attribute(self)
        }
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Attribute<'a>(&'a [(&'a str, &'a str)]);

    impl<'a> Attribute<'a> {
        pub fn is_empty(&self) -> bool {
            &self.0.len() == &0
        }
    }

    pub struct Graph<'a> {
        pub nodes: Vec<Node<'a>>,
        pub edges: Vec<Edge<'a>>,
        pub attrs: Attribute<'a>,
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: Default::default(),
            }
        }

        pub fn with_nodes(mut self, nodes: &'a Vec<Node>) -> Self {
            self.nodes = nodes.to_owned();
            self
        }
    }

    pub mod graph_items {
        use super::Attribute;
        pub mod node {
            use super::Attribute;

            #[derive(Clone, Debug, Default, PartialEq)]
            pub struct Node<'a>(&'a str, Attribute<'a>);

            impl<'a> Node<'a> {
                pub fn new(node: &'a str) -> Self {
                    Node(node, Default::default())
                }

                pub fn with_attrs(self, attrs: &'a [(&'a str, &'a str)]) -> Self {
                    Node(self.0, attrs.into())
                }
            }
        }

        pub mod edge {
            use super::Attribute;

            #[derive(Clone, Debug, Default, PartialEq)]
            pub struct Edge<'a>((&'a str, &'a str), Attribute<'a>);

            impl<'a> Edge<'a> {
                pub fn new(edge: (&'a str, &'a str)) -> Self {
                    Edge(edge, Default::default())
                }
            }
        }
    }
}
