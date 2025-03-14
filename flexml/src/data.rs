use crate::{IntoXML, XMLError, XMLNamespace};

use crate::node::XMLNode;

use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum XML {
    Container(Vec<XML>),
    Node(XMLNode),
    Text(String),
    None,
}

impl XML {
    pub fn new<T: Display>(name: T) -> Self {
        XML::Node(XMLNode::new(name))
    }

    pub fn new_untagged() -> Self {
        XML::Container(Vec::new())
    }

    #[inline]
    pub fn attribute<T: Display>(
        mut self,
        attribute_name: &'static str,
        attribute_value: T,
    ) -> Self {
        self.add_attribute(attribute_name, attribute_value);
        self
    }

    #[inline]
    pub fn add_attribute<T: Display>(&mut self, attribute_name: &'static str, attribute_value: T) {
        match self {
            XML::Node(ref mut node) => node.add_attribute(attribute_name, attribute_value),
            XML::Container(ref mut nodes) => {
                for node in nodes {
                    if let XML::Node(node) = node {
                        node.add_attribute(attribute_name, attribute_value.to_string())
                    }
                }
            }
            _ => {}
        }
    }

    #[inline]
    pub fn name<T: Display>(mut self, name: T) -> Self {
        self.set_name(name);
        self
    }

    #[inline]
    pub fn set_name<T: Display>(&mut self, name: T) {
        match self {
            XML::Node(ref mut node) => node.set_name(name),
            XML::Container(ref mut nodes) => {
                nodes.iter_mut().for_each(|n| n.set_name(name.to_string()))
            }
            _ => {}
        }
    }

    pub fn case<T: Display>(mut self, case: T) -> Self {
        self.set_case(case);
        self
    }

    pub fn set_case<T: Display>(&mut self, case: T) {
        match self {
            XML::Node(ref mut node) => node.set_case(case),
            XML::Container(ref mut nodes) => {
                nodes.iter_mut().for_each(|n| n.set_case(case.to_string()))
            }
            _ => {}
        }
    }

    #[inline]
    pub fn namespace(mut self, namespace: &'static str) -> Result<Self, XMLError> {
        self.set_namespace(namespace)?;
        Ok(self)
    }

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

    pub fn namespaces(&self) -> Vec<XMLNamespace> {
        match self {
            XML::Node(node) => node.namespaces(),
            XML::Container(nodes) => nodes
                .iter()
                .flat_map(|n| n.namespaces())
                .collect::<Vec<XMLNamespace>>(),
            _ => Vec::new(),
        }
    }

    #[inline]
    pub fn data(mut self, data: &[XML]) -> Self {
        self.add_data(data);
        self
    }

    #[inline]
    pub fn add_data<T: IntoXML>(&mut self, data: &[T]) {
        match self {
            XML::Node(ref mut node) => node.add_data(data),
            XML::Container(ref mut nodes) => {
                let mut data = data.iter().map(|d| d.to_xml()).collect();
                nodes.append(&mut data)
            }
            XML::Text(t) => {
                *self = XML::Container(vec![t.to_xml()]);
                self.add_data(data);
            }
            _ => {}
        }
    }

    #[inline]
    pub fn datum<T: IntoXML>(mut self, datum: T) -> Self {
        self.add_datum(datum);
        self
    }

    #[inline]
    pub fn add_datum<T: IntoXML>(&mut self, datum: T) {
        match self {
            XML::Node(ref mut node) => node.add_datum(datum),
            XML::Container(ref mut nodes) => nodes.push(datum.to_xml()),
            XML::Text(t) => {
                *self = XML::Container(vec![t.to_xml()]);
                self.add_datum(datum);
            }
            _ => {}
        }
    }

    #[inline]
    pub fn node(mut self, node: XML) -> Self {
        self.add_datum(node);
        self
    }

    #[inline]
    pub fn add_node(&mut self, node: XML) {
        self.add_datum(node)
    }

    #[inline]
    pub fn nodes(mut self, nodes: &[XML]) -> Self {
        self.add_nodes(nodes);
        self
    }

    #[inline]
    pub fn add_nodes(&mut self, nodes: &[XML]) {
        self.add_data(nodes)
    }

    #[inline]
    pub fn text(mut self, text: &String) -> Self {
        self.add_datum(text.to_xml());
        self
    }

    #[inline]
    pub fn add_text(&mut self, text: &String) {
        self.add_datum(text.to_xml())
    }

    pub fn sub_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(s) => write!(f, "{s}"),
            Self::Node(node) => node.sub_fmt(f),
            Self::Container(nodes) => {
                for node in nodes {
                    node.sub_fmt(f)?
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
                    node.fmt(f)?
                }
                Ok(())
            }
            Self::None => Ok(()),
        }
    }
}

impl From<XMLNode> for XML {
    fn from(value: XMLNode) -> Self {
        XML::Node(value)
    }
}

impl IntoXML for XML {
    fn to_xml(&self) -> XML {
        self.to_owned()
    }
}
