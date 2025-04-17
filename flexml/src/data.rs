use crate::{IntoXML, XMLNamespaces};

use crate::node::XMLNode;

use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum XML<'a> {
    Container(Vec<XML<'a>>),
    Node(XMLNode<'a>),
    Text(String),
    None,
}

impl<'a> XML<'a> {
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
            XML::Text(s) => *self = Self::new(name).text(s),
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
    pub fn namespace<T: Display>(
        mut self,
        namespace: &T,
        namespaces: &'a mut XMLNamespaces,
    ) -> Self {
        self.set_namespace(namespace, namespaces);
        self
    }

    #[inline]
    pub fn set_namespace<T: Display>(&mut self, namespace: &T, namespaces: &'a mut XMLNamespaces) {
        match self {
            Self::Node(ref mut node) => node.set_namespace(namespace, namespaces),
            Self::Container(_) => {} // Consider ' return Err(XMLError::NamespaceOnContainer '
            Self::Text(_) => {}      // Consider ' return Err(XMLError::NamespaceOnText), '
            Self::None => (),
        }
    }

    pub fn namespaces(&self) -> XMLNamespaces {
        match self {
            XML::Node(node) => node.determine_namespaces(),
            XML::Container(nodes) => {
                let mut namespaces = XMLNamespaces::new();
                for node in nodes.iter() {
                    node.namespaces_recurse(&mut namespaces);
                }
                namespaces
            }
            _ => XMLNamespaces::new(),
        }
    }

    pub(crate) fn namespaces_recurse(&self, namespaces: &mut XMLNamespaces) {
        match self {
            XML::Node(node) => node.namespaces_recurse(namespaces),
            XML::Container(nodes) => {
                for node in nodes.iter() {
                    node.namespaces_recurse(namespaces);
                }
            }
            _ => {}
        }
    }

    #[inline]
    pub fn data<T: IntoXML<'a>>(mut self, data: &'a [T]) -> Self {
        self.add_data(data);
        self
    }

    #[inline]
    pub fn add_data<T: IntoXML<'a>>(&mut self, data: &'a [T]) {
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
    pub fn datum<T: IntoXML<'a> + 'a>(mut self, datum: T) -> Self {
        self.add_datum(datum);
        self
    }

    #[inline]
    pub fn add_datum<T: IntoXML<'a> + 'a>(&mut self, datum: T) {
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
    pub fn node(mut self, node: XML<'a>) -> Self {
        self.add_datum(node);
        self
    }

    #[inline]
    pub fn add_node(&mut self, node: XML<'a>) {
        self.add_datum(node)
    }

    #[inline]
    pub fn nodes(mut self, nodes: &'a [XML<'a>]) -> Self {
        self.add_nodes(nodes);
        self
    }

    #[inline]
    pub fn add_nodes(&mut self, nodes: &'a [XML<'a>]) {
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

impl Display for XML<'_> {
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

impl<'a> From<XMLNode<'a>> for XML<'a> {
    fn from(value: XMLNode<'a>) -> Self {
        XML::Node(value)
    }
}

impl<'a> IntoXML<'a> for XML<'a> {
    fn to_xml(&self) -> XML<'a> {
        self.to_owned()
    }
}
