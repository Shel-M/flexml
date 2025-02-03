pub use crate::XMLActions;
use crate::XMLNamespace;

use super::XMLData;

use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug)]
pub struct XMLNode {
    name: &'static str,
    namespace: Option<XMLNamespace>,

    attributes: HashMap<&'static str, String>,
    data: XMLData,
}

impl XMLNode {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            namespace: None,

            attributes: HashMap::new(),
            data: XMLData::None,
        }
    }
}

impl XMLActions for XMLNode {
    fn get_attributes_mut(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.attributes
    }

    fn get_data(&self) -> &XMLData {
        &self.data
    }

    fn get_data_mut(&mut self) -> &mut XMLData {
        &mut self.data
    }

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_namespace(&self) -> &Option<XMLNamespace> {
        &self.namespace
    }

    fn get_namespace_mut(&mut self) -> &mut Option<XMLNamespace> {
        &mut self.namespace
    }
}

impl Display for XMLNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ns_tag = match &self.namespace {
            Some(ns) => format!("{}:{}", ns.alias, self.name),
            None => format!("{}", self.name),
        };

        write!(f, "<{ns_tag}")?;

        for attribute in &self.attributes {
            write!(f, r#" {}="{}""#, attribute.0, attribute.1)?;
        }
        match &self.data {
            XMLData::None => write!(f, "/>")?,
            d => {
                write!(f, ">")?;
                d.fmt(f)?;
                write!(f, "</{ns_tag}>")?;
            }
        }

        Ok(())
    }
}
