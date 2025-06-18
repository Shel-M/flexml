use std::fmt::Display;

use crate::{
    namespaces_v2::{XMLNamespace, XMLNamespaces},
    XMLError,
};

#[derive(Debug, Default)]
pub struct XMLDocument {
    name: String,
    namespaces: Option<XMLNamespaces>,
    namespace: Option<String>,
    // attributes: Vec<XMLAttributes>
    // child_nodes: Vec<Box<XMLNode>>
}

impl XMLDocument {
    pub fn new<T: Display>(name: T) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    /**
     Add a namespace to the internal XMLNamespaces collection, returning a reference to the new key.
    */
    pub fn define_namespace<T: Display, V: Display>(&mut self, name: T, uri: V) -> &String {
        let namespaces = match self.namespaces {
            Some(ref mut ns) => ns,
            None => self.namespaces.insert(XMLNamespaces::new()),
        };

        namespaces.insert(name, uri)
    }

    /**
    Set the namespace to be applied to this node.
    Returns an error if the String passed in to the `name` parameter is not a valid key defined by a call to define_namespace.
    */
    pub fn set_namespace(&mut self, name: String) -> Result<(), XMLError> {
        let Some(ref mut namespaces) = self.namespaces else {
            return Err(XMLError::NamespaceNotFound(name));
        };

        if namespaces.contains(&name) {
            self.namespace = Some(name);
            Ok(())
        } else {
            Err(XMLError::NamespaceNotFound(name))
        }
    }

    /**
    Set the namespace to be applied to this node.
    Returns an error if the String passed in to the `name` parameter is not a valid key defined by a call to define_namespace.

    Takes and returns Self to enable method chaining.
    */
    pub fn namespace(mut self, name: String) -> Result<Self, XMLError> {
        self.set_namespace(name)?;
        Ok(self)
    }
}

impl Display for XMLDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<")?;

        if let Some(namespaces) = &self.namespaces {
            if let Some(ns) = &self.namespace {
                let default_namespace = XMLNamespace::default();
                let namespace = namespaces.get(ns).unwrap_or(&default_namespace);
                write!(f, "{}:", namespace.alias)?;
            }

            write!(f, "{}", self.name)?;
            for (_, namespace) in &namespaces._inner {
                // todo: Limit rendering namespaces to only those that need be rendered.
                write!(f, r#" xmlns:{}="{}""#, namespace.alias, namespace.uri)?;
            }
        } else {
            write!(f, "{}", self.name)?;
        }

        // todo: write out attributes

        write!(f, ">")
    }
}
