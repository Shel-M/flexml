use log::warn;

use crate::attribute::XMLAttribute;
use crate::conv_case;
use crate::{IntoXML, XMLError, XMLNamespace, XMLNamespaces, XML};

use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct XMLNode {
    name: String,
    namespace: Option<XMLNamespace>,

    attributes: Vec<XMLAttribute>,
    data: Vec<XML>,
}

impl XMLNode {
    pub fn new<T: Display>(name: T) -> Self {
        Self {
            name: name.to_string(),
            namespace: None,

            attributes: Vec::new(),
            data: Vec::new(),
        }
    }

    #[inline]
    pub fn attribute(mut self, attribute: XMLAttribute) -> Self {
        self.add_attribute(attribute);
        self
    }

    #[inline]
    pub fn add_attribute(&mut self, attribute: XMLAttribute) {
        if self.attributes.iter().any(|a| a.key == attribute.key) {
            warn!(
                "Duplicate attribute {} added to {}. Overwriting.",
                attribute.key, self.name
            );
        }
        self.attributes.push(attribute);
    }

    #[inline]
    pub fn name<T: Display>(mut self, name: T) -> Self {
        self.set_name(name);
        self
    }

    #[inline]
    pub fn set_name<T: Display>(&mut self, name: T) {
        self.name = name.to_string();
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
        if let Some(ns) = &self.namespace {
            ret.push(ns.clone());
        }

        for datum in &self.data {
            ret.append(&mut datum.namespaces());
        }

        for attrib in &self.attributes {
            if let Some(ns) = &attrib.namespace {
                ret.push(ns.clone());
            }
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
                .map(IntoXML::to_xml)
                .collect::<Vec<XML>>()
                .as_slice(),
        );
    }

    #[allow(clippy::needless_pass_by_value)] // May or may not be a ref, generic
    #[inline]
    pub fn datum<T: IntoXML>(mut self, datum: T) -> Self {
        self.add_datum(datum);
        self
    }

    #[allow(clippy::needless_pass_by_value)] // May or may not be a ref, generic
    #[inline]
    pub fn add_datum<T: IntoXML>(&mut self, datum: T) {
        self.data.push(datum.to_xml());
    }

    #[inline]
    pub fn node(mut self, node: Self) -> Self {
        self.add_datum(XML::Node(node));
        self
    }

    #[inline]
    pub fn add_node(&mut self, node: Self) {
        self.add_datum(XML::Node(node));
    }

    #[inline]
    pub fn nodes(mut self, nodes: &[Self]) -> Self {
        self.add_nodes(nodes);
        self
    }

    #[inline]
    pub fn add_nodes(&mut self, nodes: &[Self]) {
        self.data.extend(nodes.iter().cloned().map(XML::Node));
    }

    #[inline]
    pub fn text<T: Display>(mut self, text: &T) -> Self {
        self.add_datum(text.to_string().to_xml());
        self
    }

    #[inline]
    pub fn add_text<T: Display>(&mut self, text: &T) {
        self.add_datum(text.to_string().to_xml());
    }

    pub fn sub_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ns_tag = self.namespace.as_ref().map_or_else(
            || self.name.clone(),
            |ns| format!("{}:{}", ns.alias, self.name),
        );

        write!(f, "<{ns_tag}")?;

        for attribute in &self.attributes {
            let ns_tag = attribute.namespace.as_ref().map_or_else(
                || attribute.key.clone(),
                |ns| format!("{}:{}", ns.alias, attribute.key),
            );
            write!(f, r#" {ns_tag}="{}""#, attribute.value)?;
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
        let ns_tag = self.namespace.as_ref().map_or_else(
            || self.name.clone(),
            |ns| format!("{}:{}", ns.alias, self.name),
        );

        write!(f, "<{ns_tag}")?;

        for attribute in &self.attributes {
            let ns_tag = attribute.namespace.as_ref().map_or_else(
                || attribute.key.clone(),
                |ns| format!("{}:{}", ns.alias, attribute.key),
            );
            write!(f, r#" {ns_tag}="{}""#, attribute.value)?;
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
