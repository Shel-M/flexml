use std::{
    collections::HashMap,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use log::{error, warn};
use once_cell::sync::Lazy;

use crate::XMLError;

type NamespaceMap = HashMap<String, XMLNamespace>;

static NAMESPACES: Lazy<RwLock<NamespaceMap>> = Lazy::new(|| RwLock::new(HashMap::new()));

fn read_global() -> Result<RwLockReadGuard<'static, NamespaceMap>, XMLError> {
    match NAMESPACES.read() {
        Ok(v) => Ok(v),
        Err(e) => {
            error!("Could not lock global namespaces!");
            Err(e.into())
        }
    }
}

fn write_global() -> Result<RwLockWriteGuard<'static, NamespaceMap>, XMLError> {
    match NAMESPACES.write() {
        Ok(v) => Ok(v),
        Err(e) => {
            error!("Could not lock global namespaces!");
            Err(e.into())
        }
    }
}

#[derive(Debug)]
pub struct XMLNamespaces;

impl XMLNamespaces {
    pub fn hashmap() -> Result<NamespaceMap, XMLError> {
        let ns = read_global()?;
        Ok(ns.clone())
    }

    pub fn get(namespace: &String) -> Result<Option<XMLNamespace>, XMLError> {
        let namespaces = read_global()?;

        for (key_name, value_namespace) in namespaces.iter() {
            if key_name == namespace {
                return Ok(Some(value_namespace.clone()));
            }
        }

        Ok(None)
    }

    pub fn insert(namespace: &'static str, uri: &'static str) -> Result<(), XMLError> {
        let mut ns = write_global()?;
        let namespace = namespace.to_string();
        let uri = uri.to_string();

        if ns.contains_key(&namespace) {
            warn!("Duplicate namespace inserted to namespaces. Ignoring.");
            return Ok(());
        }

        let mut values = ns.values();
        let mut alias = namespace.to_lowercase()[0..=0].to_string();
        loop {
            if values.any(|v| v.alias == alias) {
                alias.replace_range(.., &namespace.to_lowercase()[0..=alias.len()]);
                continue;
            }
            break;
        }

        let namespace = XMLNamespace {
            alias,
            name: namespace,
            uri,
            collection_index: 0,
        };

        ns.insert(namespace.name.clone(), namespace);
        Ok(())
    }
}

// #[derive(Debug, Clone)]
// pub struct XMLNamespaces<'a>(&'a Lazy<RwLock<HashMap<&'static str, String>>>);
//
// impl<'a> XMLNamespaces<'a> {
//     pub fn new() -> Self {
//         Self(&NAMESPACES)
//     }
//
//     pub fn namespace(mut self, name: &'static str, uri: String) -> Result<Self, XMLError> {
//         let values = self.0.read()?;
//         let mut alias = name.to_lowercase()[0..=0].to_string();
//         loop {
//             if values.values().find(|v| v.alias == alias).is_some() {
//                 alias.replace_range(.., &name.to_lowercase()[0..=alias.len()]);
//                 continue;
//             }
//             break;
//         }
//
//         let namespace = XMLNamespace {
//             alias: alias[0..=0].to_string(),
//             name,
//             uri,
//         };
//         if self.0.contains_key(&name) {
//             warn!("Duplicate namespace key pushed to XMLDoc.namespaces. Overwriting.")
//         }
//         self.0.insert(name, namespace);
//         Ok(self)
//     }
//
//     pub fn get(&self, name: &'static str) -> Option<XMLNamespace> {
//         if let Some(ns) = self.0.get(name) {
//             Some(ns.clone())
//         } else {
//             None
//         }
//     }
// }
//
// type NamespaceMap = HashMap<&'static str, XMLNamespace>;
// impl<'a> IntoIterator for &'a XMLNamespaces<'_> {
//     type Item = <&'a NamespaceMap as IntoIterator>::Item;
//
//     type IntoIter = <&'a NamespaceMap as IntoIterator>::IntoIter;
//
//     fn into_iter(self) -> Self::IntoIter {
//         (&self.0).into_iter()
//     }
// }
//

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct XMLNamespace {
    pub alias: String,
    pub name: String,
    pub uri: String,
    pub(crate) collection_index: usize,
}
