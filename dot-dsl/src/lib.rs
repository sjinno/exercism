pub mod graph {
    // pub mod graph_items;
    // pub use graph_items::*;
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;
    use std::collections::HashMap;

    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Attribute<'a> {
        attr: &'a [(&'a str, &'a str)],
    }

    impl<'a> Attribute<'a> {
        pub fn is_empty(&self) -> bool {
            self.attr.len() == 0
        }
    }

    impl<'a> Into<Attribute<'a>> for &'a [(&'a str, &'a str)] {
        fn into(self) -> Attribute<'a> {
            Attribute { attr: self }
        }
    }

    impl<'a> PartialEq<Attribute<'a>> for HashMap<String, String> {
        fn eq(&self, other: &Attribute<'a>) -> bool {
            for (key, value) in self {
                if key != other.attr[0].0 || value != other.attr[0].1 {
                    return false;
                }
            }
            true
        }
    }

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

        pub fn with_nodes(mut self, nodes: &'a Vec<Node<'a>>) -> Self {
            self.nodes = nodes.to_owned();
            self
        }

        pub fn with_edges(mut self, edges: &'a Vec<Edge<'a>>) -> Self {
            self.edges = edges.to_owned();
            self
        }

        pub fn with_attrs(mut self, attrs: &'a [(&'a str, &'a str)]) -> Self {
            self.attrs = attrs.into();
            self
        }

        pub fn get_node(&self, node: &'a str) -> Result<(), &'a str> {
            let index = self.nodes.iter().position(|n| n.node == node).unwrap();
            // println!("{}", index);
            Ok(())
        }
    }

    pub mod graph_items {
        pub mod node {
            #[derive(Clone, Debug, PartialEq)]
            pub struct Node<'a> {
                pub node: &'a str,
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

// pub trait PartialEq<Rhs = Self>
// where
//     Rhs: ?Sized,
// {
//     fn eq(&self, other: &Rhs) -> bool;

//     fn ne(&self, other: &Rhs) -> bool {
//         !self.eq(other)
//     }
// }

// pub trait PartialEq<Rhs: ?Sized = Self> {
//     fn eq(&self, other: &Rhs) -> bool;
//     fn ne(&self, other: &Rhs) -> bool {
//         !self.eq(other)
//     }
// }

// impl PartialEq<&[(&str, &str)]> for HashMap<String, String> {
//     fn eq(&self, other: &&[(&str, &str)]) -> bool {
//         for (key, value) in self {
//             if key != other[0].0 || value != other[0].1 {
//                 return false;
//             }
//         }
//         true
//     }
// }

// impl<'a> PartialEq<Graph<'a>> for HashMap<String, String> {
//     fn eq(&self, other: &Graph) -> bool {
//         for (key, value) in self {
//             if key != other.attrs[0].0 || value != other.attrs[0].1 {
//                 return false;
//             }
//         }
//         true
//     }
// }

// impl<'a> PartialEq<HashMap<String, String>> for Graph<'a> {
//     fn eq(&self, other: &HashMap<String, String>) -> bool {
//         for (key, value) in other {
//             if key != self.attrs[0].0 || value != self.attrs[0].1 {
//                 return false;
//             }
//         }
//         true
//     }
// }

// impl<'a> PartialEq<HashMap<String, String>> for Attribute<'a> {
//     fn eq(&self, other: &HashMap<String, String>) -> bool {
//         for (key, value) in other {
//             if key != self.attr[0].0 || value != self.attr[0].1 {
//                 return false;
//             }
//         }
//         true
//     }
// }
