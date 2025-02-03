pub mod data;
pub mod doc;
pub mod namespace;
pub mod node;

pub use data::*;
pub use doc::*;
pub use namespace::*;
pub use node::*;

use std::{
    collections::HashMap,
    error::Error,
    fmt::{self, Display},
    vec,
};

use log::warn;

pub trait XMLActions: Sized {
    fn get_attributes_mut(&mut self) -> &mut HashMap<&'static str, String>;
    fn get_data(&self) -> &XMLData;
    fn get_data_mut(&mut self) -> &mut XMLData;
    fn get_name(&self) -> String;
    fn get_namespace(&self) -> &Option<XMLNamespace>;
    fn get_namespace_mut(&mut self) -> &mut Option<XMLNamespace>;

    fn attribute(mut self, attribute_name: &'static str, attribute_value: &String) -> Self {
        self.add_attribute(attribute_name, attribute_value);
        self
    }

    fn add_attribute(&mut self, attribute_name: &'static str, attribute_value: &String) {
        let attr = self.get_attributes_mut();
        if attr.contains_key(&attribute_name) {
            warn!("Duplicate attribute pushed to XMLDoc.attributes. Overwriting.");
        }
        attr.insert(attribute_name, attribute_value.to_string());
    }

    fn namespace(mut self, namespace: &'static str) -> Result<Self, XMLError> {
        self.set_namespace(namespace)?;
        Ok(self)
    }

    fn set_namespace(&mut self, namespace: &'static str) -> Result<(), XMLError> {
        if let Some(ns) = XMLNamespaces::get(&namespace.to_string())? {
            let n = self.get_namespace_mut();

            _ = n.insert(ns);
        } else {
            warn!("Namespace {namespace} not defined.");
            return Err(XMLError::NamespaceNotFound(namespace.to_string()));
        }
        Ok(())
    }

    fn namespaces(&self) -> Vec<XMLNamespace> {
        let data = self.get_data();
        let namespace = self.get_namespace();

        let mut ret = Vec::new();
        if namespace.is_some() {
            ret.push(namespace.clone().unwrap());
        }

        if let XMLData::Nodes(nodes) = data {
            for node in nodes {
                ret.append(&mut node.namespaces())
            }
        }

        ret.sort();
        ret.dedup();
        ret
    }

    fn node(mut self, node: XMLNode) -> Self {
        self.add_node(node);
        self
    }

    fn add_node(&mut self, node: XMLNode) {
        let name = self.get_name();
        let data = self.get_data_mut();
        match data {
            XMLData::None => *data = XMLData::Nodes(vec![node]),
            XMLData::Nodes(ref mut vec) => {
                vec.push(node);
            }
            XMLData::Text(_) => {
                warn!(
                    "XML Node for '{}' already contains flat text data. Overwriting.",
                    name
                );
                *data = XMLData::Nodes(vec![node])
            }
        }
    }

    fn node_text(mut self, text: String) -> Self {
        let name = self.get_name();
        let data = self.get_data_mut();
        match data {
            XMLData::None => *data = XMLData::Text(text),
            XMLData::Nodes(_) => {
                warn!(
                    "XML Node for {} already contains child node data. *NOT* Overwriting.",
                    name
                );
                return self;
            }
            XMLData::Text(_) => *data = XMLData::Text(text),
        }
        self
    }

    fn set_node_text(&mut self, text: String) {
        let name = self.get_name();
        let data = self.get_data_mut();
        match data {
            XMLData::None => *data = XMLData::Text(text),
            XMLData::Nodes(_) => {
                warn!(
                    "XML Node for {} already contains child node data. *NOT* Overwriting.",
                    name
                );
            }
            XMLData::Text(_) => *data = XMLData::Text(text),
        }
    }
}

pub trait IntoXMLNode {
    fn into_xml(&self) -> XMLNode;
}

pub trait IntoXMLDoc {
    fn into_xml(&self) -> XMLDoc;
}

#[derive(Debug)]
pub enum XMLError {
    NamespaceNotFound(String),
    Other(String),
}

impl Display for XMLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        type S = XMLError;

        match self {
            S::NamespaceNotFound(v) => write!(
                f,
                "XMLError::NamespaceNotFound - Namespace \"{v}\" not defined"
            ),
            S::Other(v) => write!(f, "XMLError::Other \"{v}\""),
        }
    }
}

impl<T: Error + Display> From<T> for XMLError {
    fn from(value: T) -> Self {
        Self::Other(value.to_string())
    }
}

#[cfg(test)]
mod test {

    use crate::XMLNamespaces;

    use super::{IntoXMLDoc, IntoXMLNode, XMLActions, XMLDoc, XMLNode};

    struct Root {
        data1: Node,
        data2: Node,
        attrib1: String,
    }

    impl IntoXMLDoc for Root {
        fn into_xml(&self) -> XMLDoc {
            XMLNamespaces::insert("Namespace1", "https://namespace1.com/namespace")
                .expect("failed to insert namespace");

            XMLNamespaces::insert("Namespace2", "https://namespace2.com/namespace")
                .expect("failed to insert namespace");

            let mut doc = XMLDoc::new("root")
                .attribute("attrib1", &self.attrib1)
                .namespace("Namespace1")
                .expect("Failed to set doc namespace")
                // If the node does not need additional attributes or namespaces, you can method chain to
                // add it onto the doc in-place
                .node(
                    self.data1
                        .into_xml()
                        .namespace("Namespace1")
                        .expect("Could not set namespace"),
                );

            // If you need namespaces or attributes, you can create a variable, mutate, and then
            // add into the doc afterwards.
            let mut d2_node = self.data2.into_xml();
            d2_node
                .set_namespace("Namespace1")
                .expect("Could not set namespace");
            doc.add_node(d2_node);

            doc
        }
    }

    struct Node {
        data1: Option<String>,
        data2: Option<Box<Node>>,
    }

    impl IntoXMLNode for Node {
        fn into_xml(&self) -> XMLNode {
            let mut node = XMLNode::new("Node");

            match (&self.data1, &self.data2) {
                (None, None) => {}
                (Some(d), None) => node.set_node_text(d.to_string()), // set_node_text is for data
                (None, Some(d)) => node.add_node(d.into_xml()),       // add_node is for child nodes
                (Some(_), Some(_)) => {
                    panic!("Both data values are set, incompatible implementation")
                }
            }

            node
        }
    }

    #[test]
    fn test_build_simple_xml() {
        let test_structure = Root {
            data1: Node {
                data1: Some("First node, first datapoint".to_string()),
                data2: None,
            },
            data2: Node {
                data1: None,
                data2: Some(Box::new(Node {
                    data1: Some("Second node, sub-datapoint".to_string()),
                    data2: None,
                })),
            },
            attrib1: "Attribute_value".to_string(),
        };

        assert_eq!(
            r#"<n:root attrib1="Attribute_value" xmlns:n="https://namespace1.com/namespace"><Node>First node, first datapoint</Node><n:Node><Node>Second node, sub-datapoint</Node></n:Node></n:root>"#,
            test_structure.into_xml().to_string()
        )
    }
}
