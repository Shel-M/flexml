use crate::{IntoXMLNode, XMLError};

use super::XMLNode;

use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum XMLData {
    Node(XMLNode),
    Text(String),
    None,
}

impl XMLData {
    pub fn namespace(self, namespace: &'static str) -> Result<Self, XMLError> {
        match self {
            Self::Node(node) => Ok(node.namespace(namespace)?.into()),
            Self::Text(_) => Err(XMLError::NamespaceOnText),
            Self::None => Ok(self),
        }
    }

    pub fn sub_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(s) => write!(f, "{s}"),
            Self::Node(node) => node.sub_fmt(f),
            Self::None => Ok(()),
        }
    }
}

impl Display for XMLData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(s) => write!(f, "{s}"),
            Self::Node(node) => node.fmt(f),
            Self::None => Ok(()),
        }
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

impl<T: IntoXMLNode> ToXMLData for T {
    fn to_xml_data(&self) -> XMLData {
        XMLData::Node(self.to_xml())
    }
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

impl ToXMLData for &String {
    fn to_xml_data(&self) -> XMLData {
        XMLData::Text(self.to_string())
    }
}

impl ToXMLData for u8 {
    fn to_xml_data(&self) -> XMLData {
        XMLData::Text(self.to_string())
    }
}

impl ToXMLData for u16 {
    fn to_xml_data(&self) -> XMLData {
        XMLData::Text(self.to_string())
    }
}

impl ToXMLData for u32 {
    fn to_xml_data(&self) -> XMLData {
        XMLData::Text(self.to_string())
    }
}

impl ToXMLData for u64 {
    fn to_xml_data(&self) -> XMLData {
        XMLData::Text(self.to_string())
    }
}

impl ToXMLData for u128 {
    fn to_xml_data(&self) -> XMLData {
        XMLData::Text(self.to_string())
    }
}

impl ToXMLData for i8 {
    fn to_xml_data(&self) -> XMLData {
        XMLData::Text(self.to_string())
    }
}

impl ToXMLData for i16 {
    fn to_xml_data(&self) -> XMLData {
        XMLData::Text(self.to_string())
    }
}

impl ToXMLData for i32 {
    fn to_xml_data(&self) -> XMLData {
        XMLData::Text(self.to_string())
    }
}

impl ToXMLData for i64 {
    fn to_xml_data(&self) -> XMLData {
        XMLData::Text(self.to_string())
    }
}

impl ToXMLData for i128 {
    fn to_xml_data(&self) -> XMLData {
        XMLData::Text(self.to_string())
    }
}
