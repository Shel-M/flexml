use std::fmt::Display;

use log::warn;

use crate::{XMLError, XMLNamespace, XMLNamespaces};

#[derive(Debug, Clone)]
pub struct XMLAttribute {
    pub(crate) key: String,
    pub(crate) value: String,
    pub(crate) namespace: Option<XMLNamespace>,
}

impl XMLAttribute {
    pub fn new<T: Display, V: Display>(key: T, value: V) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
            namespace: None,
        }
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
}
