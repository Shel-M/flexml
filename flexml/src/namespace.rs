use indexmap::IndexMap;

#[derive(Debug, Clone)]
pub struct XMLNamespaces {
    pub(crate) _inner: IndexMap<String, XMLNamespace>,
    longest_key: usize,
}

impl XMLNamespaces {
    pub fn new() -> Self {
        Self {
            _inner: IndexMap::new(),
            longest_key: 1,
        }
    }

    /// Returns borrowed reference to namespace by `key`, None if it does not exist.
    pub fn get(&self, key: &String) -> Option<&XMLNamespace> {
        self._inner.get(key)
    }

    pub fn insert(&mut self, mut namespace: XMLNamespace) -> &String {
        let name = namespace.name.clone();

        let alias = if let Some(key) = self.contains_uri(&namespace) {
            self._inner.insert(key.clone(), namespace);
            return self._inner.get_key_value(&key).unwrap().0;
        } else {
            let mut a = String::with_capacity(self.longest_key);
            for ch in name.chars() {
                a.push(ch);
                if self._inner.contains_key(&a) {
                    if a.len() + 1 > self.longest_key {
                        self.longest_key = a.len();
                    }

                    continue;
                }
                break;
            }
            a
        };

        namespace.alias = alias.clone();
        let ret_alias = alias.clone();
        self._inner.insert(alias, namespace);

        self._inner.get_key_value(&ret_alias).unwrap().0
    }

    pub fn contains_uri(&self, namespace: &XMLNamespace) -> Option<String> {
        for (k, v) in self._inner.iter() {
            if v.uri == namespace.uri {
                return Some(k.to_string());
            }
        }

        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct XMLNamespace {
    pub alias: String,
    pub name: String,
    pub uri: String,
}

impl XMLNamespace {
    pub fn new(name: String, uri: String) -> Self {
        Self {
            alias: String::new(),
            name,
            uri,
        }
    }
}
