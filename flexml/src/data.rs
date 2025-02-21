use crate::{IntoXMLNode, XMLError};

use super::XMLNode;

use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum XMLData {
    Node(XMLNode),
    Text(String),
}

impl XMLData {
    pub fn namespace(self, namespace: &'static str) -> Result<Self, XMLError> {
        match self {
            XMLData::Node(node) => Ok(node.namespace(namespace)?.into()),
            XMLData::Text(_) => Err(XMLError::NamespaceOnText),
        }
    }

    pub fn sub_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(s) => write!(f, "{s}"),
            Self::Node(node) => node.sub_fmt(f),
        }
    }
}

impl Display for XMLData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(s) => write!(f, "{s}"),
            Self::Node(node) => node.fmt(f),
        }
    }
}

impl From<&String> for XMLData {
    fn from(value: &String) -> Self {
        XMLData::Text(value.to_string())
    }
}

impl From<XMLNode> for XMLData {
    fn from(value: XMLNode) -> Self {
        XMLData::Node(value)
    }
}

impl<T: IntoXMLNode> From<&T> for XMLData {
    fn from(value: &T) -> Self {
        XMLData::Node(value.to_xml())
    }
}

pub trait ToXMLData {
    fn to_xml_data(&self) -> XMLData;
}

// Explicit implementations for built-in string types.
impl ToXMLData for &str {
    fn to_xml_data(&self) -> XMLData {
        XMLData::Text(self.to_string())
    }
}

impl ToXMLData for String {
    fn to_xml_data(&self) -> XMLData {
        // No need to clone here if you’re okay with moving it;
        // if not, you can clone as shown.
        XMLData::Text(self.clone())
    }
}

impl<T: IntoXMLNode> ToXMLData for T {
    fn to_xml_data(&self) -> XMLData {
        XMLData::Node(self.to_xml())
    }
}
