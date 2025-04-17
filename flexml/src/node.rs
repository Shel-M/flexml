use indexmap::IndexMap;
use log::warn;

use crate::{conv_case, XMLNamespace};
use crate::{IntoXML, XMLError, XMLNamespaces, XML};

use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct XMLNode<'a> {
    name: String,
    namespace: Option<&'a XMLNamespace>,
    attributes: IndexMap<String, String>,
    data: Vec<XML<'a>>,
}

impl<'a, 'b: 'a> XMLNode<'a> {
    pub fn new<T: Display>(name: T) -> Self {
        Self {
            name: name.to_string(),
            namespace: None,
            attributes: IndexMap::new(),
            data: Vec::new(),
        }
    }

    #[inline]
    pub fn attribute<T: Display, V: Display>(
        mut self,
        attribute_name: T,
        attribute_value: V,
    ) -> Self {
        self.add_attribute(attribute_name, attribute_value);
        self
    }

    #[inline]
    pub fn add_attribute<T: Display, V: Display>(&mut self, attribute_name: T, attribute_value: V) {
        let attribute_name = attribute_name.to_string();
        if self.attributes.contains_key(&attribute_name) {
            warn!(
                "Duplicate attribute {attribute_name} added to {}. Overwriting.",
                self.name
            );
        }
        self.attributes
            .insert(attribute_name, attribute_value.to_string());
    }

    #[inline]
    pub fn name<T: Display>(mut self, name: T) -> Self {
        self.set_name(name);
        self
    }

    #[inline]
    pub fn set_name<T: Display>(&mut self, name: T) {
        self.name = name.to_string()
    }

    #[inline]
    pub fn case<T: Display>(mut self, case: T) -> Self {
        self.set_case(case);
        self
    }

    #[inline]
    pub fn set_case<T: Display>(&mut self, case: T) {
        self.name = conv_case(&self.name, case);
    }

    #[inline]
    pub fn namespace<T: Display>(
        mut self,
        namespace: &T,
        namespaces: &'b mut XMLNamespaces,
    ) -> Self {
        self.set_namespace(namespace, namespaces);
        self
    }

    #[inline]
    pub fn set_namespace<T: Display>(&mut self, namespace: &T, namespaces: &'b mut XMLNamespaces) {
        let namespace = namespace.to_string();
        match namespaces.get(&namespace) {
            Some(ns) => {
                let _ = self.namespace.insert(ns);
            }
            None => {}
        };
    }

    pub fn determine_namespaces(&self) -> XMLNamespaces {
        let mut namespaces = XMLNamespaces::new();
        if let Some(namespace) = self.namespace {
            namespaces.insert(namespace.clone());
        }

        for datum in &self.data {
            datum.namespaces_recurse(&mut namespaces)
        }

        namespaces
    }

    pub(crate) fn namespaces_recurse(&self, mut namespaces: &mut XMLNamespaces) {
        if let Some(namespace) = self.namespace {
            namespaces.insert(namespace.clone());
        }

        for datum in &self.data {
            datum.namespaces_recurse(&mut namespaces)
        }
    }

    #[inline]
    pub fn data<T: IntoXML<'a> + 'b>(mut self, data: &'b [T]) -> Self {
        self.add_data(data);
        self
    }

    #[inline]
    pub fn add_data<T: IntoXML<'a> + 'b>(&mut self, data: &'b [T]) {
        self.data.extend_from_slice(
            data.iter()
                .map(|d| d.to_xml())
                .collect::<Vec<XML>>()
                .as_slice(),
        );
    }

    #[inline]
    pub fn datum<T: IntoXML<'a> + 'b>(mut self, datum: T) -> Self {
        self.add_datum(datum);
        self
    }

    #[inline]
    pub fn add_datum<T: IntoXML<'a> + 'b>(&mut self, datum: T) {
        self.data.push(datum.to_xml())
    }

    #[inline]
    pub fn node(mut self, node: XMLNode<'a>) -> Self {
        self.add_datum(XML::Node(node));
        self
    }

    #[inline]
    pub fn add_node(&mut self, node: XMLNode<'a>) {
        self.add_datum(XML::Node(node))
    }

    #[inline]
    pub fn nodes(mut self, nodes: &[XMLNode<'a>]) -> Self {
        self.add_nodes(nodes);
        self
    }

    #[inline]
    pub fn add_nodes(&mut self, nodes: &[XMLNode<'a>]) {
        self.data.extend(nodes.iter().cloned().map(XML::Node))
    }

    #[inline]
    pub fn text(mut self, text: &'b String) -> Self {
        self.add_datum(text.to_xml());
        self
    }

    #[inline]
    pub fn add_text(&mut self, text: &'b String) {
        self.add_datum(text.to_xml())
    }

    pub fn sub_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let ns_tag = self.name;
        let ns_tag = match &self.namespace {
            Some(ns) => format!("{}:{}", ns.alias, self.name),
            None => self.name.to_string(),
        };

        write!(f, "<{ns_tag}")?;

        for attribute in &self.attributes {
            write!(f, r#" {}="{}""#, attribute.0, attribute.1)?;
        }

        if self.data.is_empty() {
            write!(f, "/>")
        } else {
            write!(f, ">")?;
            for datum in &self.data {
                datum.sub_fmt(f)?;
            }
            write!(f, "</{ns_tag}>")?;
            Ok(())
        }
    }

    pub fn parse_string<T: Display>(_input: &T) -> Result<Self, XMLError> {
        todo!()
    }
}

impl<'a> IntoXML<'a> for XMLNode<'a> {
    fn to_xml(&self) -> XML<'a> {
        XML::Node(self.to_owned())
    }
}

impl Display for XMLNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let ns_tag = self.name;
        let ns_tag = match &self.namespace {
            Some(ns) => format!("{}:{}", ns.alias, self.name),
            None => self.name.to_string(),
        };

        write!(f, "<{ns_tag}")?;

        for attribute in &self.attributes {
            write!(f, r#" {}="{}""#, attribute.0, attribute.1)?;
        }

        let namespaces = self.determine_namespaces();
        for namespace in namespaces._inner.values() {
            write!(f, r#" xmlns:{}="{}""#, namespace.alias, namespace.uri)?;
        }

        if self.data.is_empty() {
            write!(f, "/>")
        } else {
            write!(f, ">")?;
            for datum in &self.data {
                datum.sub_fmt(f)?;
            }
            write!(f, "</{ns_tag}>")?;
            Ok(())
        }
    }
}
