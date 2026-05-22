use crate::attribute::XMLAttribute;
use crate::{IntoXML, XMLError, XMLNamespace};

use crate::node::XMLNode;

use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum XML {
    Container(Vec<Self>),
    Node(XMLNode),
    Text(String),
    None,
}

impl XML {
    pub fn new<T: Display>(name: T) -> Self {
        Self::Node(XMLNode::new(name))
    }

    #[must_use]
    pub const fn new_untagged() -> Self {
        Self::Container(Vec::new())
    }

    #[must_use]
    #[inline]
    pub fn attribute(mut self, attribute: XMLAttribute) -> Self {
        self.add_attribute(attribute);
        self
    }

    #[inline]
    pub fn add_attribute(&mut self, attribute: XMLAttribute) {
        match self {
            Self::Node(ref mut node) => node.add_attribute(attribute),
            Self::Container(ref mut nodes) => {
                for node in nodes {
                    if let Self::Node(node) = node {
                        node.add_attribute(attribute.clone());
                    }
                }
            }
            _ => {}
        }
    }

    #[must_use]
    #[inline]
    pub fn name<T: Display>(mut self, name: T) -> Self {
        self.set_name(name);
        self
    }

    #[inline]
    pub fn set_name<T: Display>(&mut self, name: T) {
        match self {
            Self::Node(ref mut node) => node.set_name(name),
            Self::Container(ref mut nodes) => {
                for node in nodes.iter_mut() {
                    node.set_name(name.to_string());
                }
            }
            Self::Text(s) => *self = Self::new(name).text(s),
            Self::None => {}
        }
    }

    #[must_use]
    pub fn case<T: Display>(mut self, case: T) -> Self {
        self.set_case(case);
        self
    }

    pub fn set_case<T: Display>(&mut self, case: T) {
        match self {
            Self::Node(ref mut node) => node.set_case(case),
            Self::Container(ref mut nodes) => {
                for node in nodes.iter_mut() {
                    node.set_case(case.to_string());
                }
            }
            _ => {}
        }
    }

    /// # Errors
    /// See `set_namespace`
    #[inline]
    pub fn namespace(mut self, namespace: &'static str) -> Result<Self, XMLError> {
        self.set_namespace(namespace)?;
        Ok(self)
    }

    /// # Errors
    /// Returns an error if the namespace alias passed in is not found in the global `XMLNamespaces`
    /// collection.
    #[inline]
    pub fn set_namespace(&mut self, namespace: &'static str) -> Result<(), XMLError> {
        match self {
            Self::Node(ref mut node) => node.set_namespace(namespace)?,
            Self::Container(ref mut nodes) => {
                for node in nodes {
                    node.set_namespace(namespace)?;
                }
            }
            Self::Text(_) => return Err(XMLError::NamespaceOnText),
            Self::None => (),
        }
        Ok(())
    }

    #[must_use]
    pub fn namespaces(&self) -> Vec<XMLNamespace> {
        match self {
            Self::Node(node) => node.namespaces(),
            Self::Container(nodes) => nodes
                .iter()
                .flat_map(Self::namespaces)
                .collect::<Vec<XMLNamespace>>(),
            _ => Vec::new(),
        }
    }

    #[must_use]
    #[inline]
    pub fn data(mut self, data: &[Self]) -> Self {
        self.add_data(data);
        self
    }

    #[inline]
    pub fn add_data<T: IntoXML>(&mut self, data: &[T]) {
        match self {
            Self::Node(ref mut node) => node.add_data(data),
            Self::Container(ref mut nodes) => {
                let mut data = data.iter().map(IntoXML::to_xml).collect();
                nodes.append(&mut data);
            }
            Self::Text(t) => {
                *self = Self::Container(vec![t.to_xml()]);
                self.add_data(data);
            }
            Self::None => {}
        }
    }

    #[must_use]
    #[inline]
    pub fn datum<T: IntoXML>(mut self, datum: T) -> Self {
        self.add_datum(datum);
        self
    }

    #[inline]
    pub fn add_datum<T: IntoXML>(&mut self, datum: T) {
        match self {
            Self::Node(ref mut node) => node.add_datum(datum),
            Self::Container(ref mut nodes) => nodes.push(datum.to_xml()),
            Self::Text(t) => {
                *self = Self::Container(vec![t.to_xml()]);
                self.add_datum(datum);
            }
            Self::None => {}
        }
    }

    #[must_use]
    #[inline]
    pub fn node(mut self, node: Self) -> Self {
        self.add_datum(node);
        self
    }

    #[inline]
    pub fn add_node(&mut self, node: Self) {
        self.add_datum(node);
    }

    #[must_use]
    #[inline]
    pub fn nodes(mut self, nodes: &[Self]) -> Self {
        self.add_nodes(nodes);
        self
    }

    #[inline]
    pub fn add_nodes(&mut self, nodes: &[Self]) {
        self.add_data(nodes);
    }

    #[must_use]
    #[inline]
    pub fn text(mut self, text: &String) -> Self {
        self.add_datum(text.to_xml());
        self
    }

    #[inline]
    pub fn add_text(&mut self, text: &String) {
        self.add_datum(text.to_xml());
    }

    /// # Errors
    /// See [`XMLNode::sub_fmt`]
    pub fn sub_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(s) => write!(f, "{s}"),
            Self::Node(node) => node.sub_fmt(f),
            Self::Container(nodes) => {
                for node in nodes {
                    node.sub_fmt(f)?;
                }
                Ok(())
            }
            Self::None => Ok(()),
        }
    }
}

impl Display for XML {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(s) => write!(f, "{s}"),
            Self::Node(node) => node.fmt(f),
            Self::Container(nodes) => {
                for node in nodes {
                    node.fmt(f)?;
                }
                Ok(())
            }
            Self::None => Ok(()),
        }
    }
}

impl From<XMLNode> for XML {
    fn from(value: XMLNode) -> Self {
        Self::Node(value)
    }
}

impl IntoXML for XML {
    fn to_xml(&self) -> XML {
        self.to_owned()
    }
}
