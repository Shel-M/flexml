use std::fmt::Display;

use log::info;

use crate::{
    node::{Namespace, NodeType, XMLNode},
    XMLNamespace,
};

pub struct XML {
    nodes: Vec<XMLNode>,
    namespaces: Vec<XMLNamespace>,

    cursor: usize,
}

impl XML {
    pub fn new<T: Display>(name: T) -> Self {
        Self {
            nodes: vec![XMLNode::new_typed(name, NodeType::Document)],
            namespaces: Vec::new(),

            cursor: 0,
        }
    }

    pub fn define_namespace<T: Display, V: Display>(&mut self, name: T, uri: V) {
        let collection_index = self.namespaces.len();
        let existing_aliases: Vec<&String> = self.namespaces.iter().map(|n| &n.alias).collect();

        let matching_uri: Vec<usize> = self
            .namespaces
            .iter()
            .filter(|n| n.uri == uri.to_string())
            .map(|n| n.collection_index)
            .collect();

        if !matching_uri.is_empty() {
            let matching_uri = self.namespaces.get(*matching_uri.first().unwrap()).unwrap();
            info!(
                "Duplicate namespace {name} defined as {}. Aliasing together.",
                matching_uri.name
            );

            // Add an empty namespace to alias the names together
            self.namespaces.push(XMLNamespace {
                alias: String::new(),
                name: name.to_string(),
                uri: String::new(),
                collection_index: matching_uri.collection_index,
            });
        } else {
            let mut i = 0;
            let alias = loop {
                let a = format!("{}{}", &name.to_string()[0..1], i);
                if !existing_aliases.contains(&&a) {
                    break a;
                }
                i += 1;
            };

            self.namespaces.push(XMLNamespace {
                alias,
                name: name.to_string(),
                uri: uri.to_string(),
                collection_index,
            });
        };
    }

    // Internals

    fn add_node<T: Display>(
        &mut self,
        node_type: NodeType,
        name: T,
        namespace: Namespace,
    ) -> usize {
    }
}
