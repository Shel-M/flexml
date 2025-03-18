use indexmap::IndexMap;
use log::warn;

use crate::conv_case;
use crate::{IntoXML, XMLError, XMLNamespace, XMLNamespaces, XML};

use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct XMLNode {
    name: String,
    namespace: Option<XMLNamespace>,

    attributes: IndexMap<&'static str, String>,
    data: Vec<XML>,
}

impl XMLNode {
    pub fn new<T: Display>(name: T) -> Self {
        Self {
            name: name.to_string(),
            namespace: None,

            attributes: IndexMap::new(),
            data: Vec::new(),
        }
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

    pub fn case<T: Display>(mut self, case: T) -> Self {
        self.set_case(case);
        self
    }

    pub fn set_case<T: Display>(&mut self, case: T) {
        self.name = conv_case(&self.name, case);
    }

    #[inline]
    pub fn namespace(mut self, namespace: &'static str) -> Result<Self, XMLError> {
        self.set_namespace(namespace)?;
        Ok(self)
    }

    #[inline]
    pub fn set_namespace(&mut self, namespace: &'static str) -> Result<(), XMLError> {
        if let Some(ns) = XMLNamespaces::get(&namespace.to_string())? {
            _ = self.namespace.insert(ns);
        } else {
            warn!("Namespace {namespace} not defined.");
            return Err(XMLError::NamespaceNotFound(namespace.to_string()));
        }
        Ok(())
    }

    pub fn namespaces(&self) -> Vec<XMLNamespace> {
        let mut ret = Vec::new();
        if self.namespace.is_some() {
            ret.push(self.namespace.clone().unwrap());
        }

        for datum in &self.data {
            ret.append(&mut datum.namespaces())
        }

        ret.sort();
        ret.dedup();
        ret
    }

    #[inline]
    pub fn data<T: IntoXML>(mut self, data: &[T]) -> Self {
        self.add_data(data);
        self
    }

    #[inline]
    pub fn add_data<T: IntoXML>(&mut self, data: &[T]) {
        self.data.extend_from_slice(
            data.iter()
                .map(|d| d.to_xml())
                .collect::<Vec<XML>>()
                .as_slice(),
        );
    }

    #[inline]
    pub fn datum<T: IntoXML>(mut self, datum: T) -> Self {
        self.add_datum(datum);
        self
    }

    #[inline]
    pub fn add_datum<T: IntoXML>(&mut self, datum: T) {
        self.data.push(datum.to_xml())
    }

    #[inline]
    pub fn node(mut self, node: XMLNode) -> Self {
        self.add_datum(XML::Node(node));
        self
    }

    #[inline]
    pub fn add_node(&mut self, node: XMLNode) {
        self.add_datum(XML::Node(node))
    }

    #[inline]
    pub fn nodes(mut self, nodes: &[XMLNode]) -> Self {
        self.add_nodes(nodes);
        self
    }

    #[inline]
    pub fn add_nodes(&mut self, nodes: &[XMLNode]) {
        self.data.extend(nodes.iter().cloned().map(XML::Node))
    }

    #[inline]
    pub fn text<T: Display>(mut self, text: &T) -> Self {
        self.add_datum(text.to_string().to_xml());
        self
    }

    #[inline]
    pub fn add_text<T: Display>(&mut self, text: &T) {
        self.add_datum(text.to_string().to_xml())
    }

    pub fn sub_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
}

impl IntoXML for XMLNode {
    fn to_xml(&self) -> XML {
        XML::Node(self.to_owned())
    }
}

impl Display for XMLNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ns_tag = match &self.namespace {
            Some(ns) => format!("{}:{}", ns.alias, self.name),
            None => self.name.to_string(),
        };

        write!(f, "<{ns_tag}")?;

        for attribute in &self.attributes {
            write!(f, r#" {}="{}""#, attribute.0, attribute.1)?;
        }

        let namespaces = self.namespaces();

        for namespace in namespaces {
            if let Ok(Some(namespace)) = XMLNamespaces::get(&namespace.name) {
                write!(f, r#" xmlns:{}="{}""#, namespace.alias, namespace.uri)?;
            }
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
