pub mod graph {
    use std::collections::HashMap;

    use graph_items::{edge::Edge, node::Node};

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct Edge {
                attrs: HashMap<String, String>,
                from: String,
                to: String,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Self {
                        attrs: HashMap::new(),
                        from: from.to_string(),
                        to: to.to_string(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs.extend(
                        attrs
                            .iter()
                            .map(|(key, value)| (key.to_string(), value.to_string())),
                    );
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|x| x.as_str())
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct Node {
                attrs: HashMap<String, String>,
                name: String,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs.extend(
                        attrs
                            .iter()
                            .map(|(key, value)| (key.to_string(), value.to_string())),
                    );
                    self
                }

                pub fn name(&self) -> &str {
                    &self.name
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|value| value.as_str())
                }
            }
        }
    }

    #[derive(Debug)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn default() -> Self {
            Self::new()
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend(nodes.iter().cloned());
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend(edges.iter().cloned());
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs.extend(
                attrs
                    .iter()
                    .map(|(key, value)| (key.to_string(), value.to_string())),
            );
            self
        }
        pub fn node(&self, name: &str) -> Option<Node> {
            self.nodes.iter().find(|&node| node.name() == name).cloned()
        }
    }
}
