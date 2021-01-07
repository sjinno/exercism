pub mod graph {
    // pub mod graph_items;
    // pub use graph_items::*;
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;

    pub struct Graph<'a> {
        nodes: Vec<Node>,
        edges: Vec<Edge>,
        attrs: &'a [&'a str],
    }

    impl Graph {
        pub fn new() -> Self {
            Graph
        }
    }

    pub mod graph_items {
        pub mod node {
            pub struct Node;
        }
        pub mod edge {
            pub struct Edge;
        }
    }
}
