//! Code for dealing with namespaces in RDF files.

use std::slice::Iter;

/// Namespaces object maps prefixes to namespaces.
#[derive(Clone)]
pub struct Namespaces {
    namespaces: Vec<Namespace>,
}

impl Namespaces {
    /// Create a new Namespaces struct.
    pub fn new() -> Namespaces {
        Namespaces {
            namespaces: Vec::new(),
        }
    }
    /// Append a new mapping from prefix to namespace.
    pub fn set(&mut self, prefix: &str, namespace: &str) {
        if let Some(ns) = self.namespaces.iter_mut().find(|ns| ns.prefix == prefix) {
            ns.namespace.clear();
            ns.namespace.push_str(namespace);
            return;
        }
        self.namespaces.push(Namespace {
            namespace: String::from(namespace),
            prefix: String::from(prefix),
        });
    }
    /// Append a new mapping from prefix to namespace.
    pub fn insert(&mut self, prefix: &str, namespace: String) {
        if let Some(ns) = self.namespaces.iter_mut().find(|ns| ns.prefix == prefix) {
            ns.namespace = namespace;
            return;
        }
        self.namespaces.push(Namespace {
            namespace,
            prefix: String::from(prefix),
        });
    }
    /// Find the first prefix in this Namespaces that matches the given string.
    ///
    /// The prefix is returned and the remainder of the string is also also returned.
    pub fn find_prefix<'a>(&self, iri: &'a str) -> Option<(&str, &'a str)> {
        for ns in &self.namespaces {
            if iri.starts_with(&ns.namespace) {
                return Some((&ns.prefix, &iri[ns.namespace.len()..]));
            }
        }
        None
    }
    /// Find the namespace for the given prefix.
    pub fn find_namespace(&self, prefix: &str) -> Option<&str> {
        for ns in &self.namespaces {
            if ns.prefix == prefix {
                return Some(ns.namespace.as_str());
            }
        }
        None
    }
    /// Iterator overall prefixes and namespaces.
    pub fn iter(&self) -> Iter<Namespace> {
        self.namespaces.iter()
    }
}

impl Default for Namespaces {
    fn default() -> Namespaces {
        Namespaces::new()
    }
}

/// An entry in Namespaces.
#[derive(Clone)]
pub struct Namespace {
    namespace: String,
    prefix: String,
}

impl Namespace {
    /// Get the namespace.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }
    /// Get the prefix.
    pub fn prefix(&self) -> &str {
        &self.prefix
    }
}
