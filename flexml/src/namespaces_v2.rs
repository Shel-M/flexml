use std::fmt::Display;

use indexmap::{map::IntoIter, IndexMap};
use log::info;

#[derive(Debug, Default)]
pub struct XMLNamespaces {
    // Key: XMLNamespace.name
    pub(crate) _inner: IndexMap<String, XMLNamespace>,
}

impl XMLNamespaces {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn contains(&self, name: &String) -> bool {
        self._inner.contains_key(name)
    }

    pub(crate) fn get(&self, name: &String) -> Option<&XMLNamespace> {
        self._inner.get(name)
    }

    /**
     Insert a namespace into the collection, calculating an appropriate alias from the name.
     Returns a reference to the key.
    */
    pub(crate) fn insert<T: Display, V: Display>(&mut self, name: T, uri: V) -> &String {
        let name = name.to_string();
        let uri = uri.to_string();

        if let Some(ref mut value) = self._inner.get_mut(&name) {
            info!("Inserted existing namespace. Updating stored uri.");
            value.uri = uri;
        } else {
            let existing_aliases: Vec<&String> = self._inner.values().map(|v| &v.alias).collect();

            let alias_char = &name[0..1];
            let mut alias_idx = 0;
            let mut alias = format!("{alias_char}0");
            while existing_aliases.contains(&&alias) {
                alias_idx += 1;
                alias = format!("{alias_char}{alias_idx}")
            }

            self._inner.insert(
                name.clone(),
                XMLNamespace {
                    alias,
                    name: name.clone(),
                    uri,
                },
            );
        }
        self.get_name_ref(&name)
    }

    fn get_name_ref(&self, name: &String) -> &String {
        let idx = self
            ._inner
            .get_index_of(name)
            .expect("Could not get index of new entry. Should not be possible");
        &self
            ._inner
            .get_index(idx)
            .expect("Could not find index of new entry. Should not be possible.")
            .1
            .name
    }
}

#[derive(Debug, Default)]
pub(crate) struct XMLNamespace {
    pub alias: String,
    pub name: String,
    pub uri: String,
}
