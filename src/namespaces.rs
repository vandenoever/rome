use std::slice::Iter;

#[derive (Clone)]
pub struct Namespaces {
    namespaces: Vec<Namespace>,
}

impl Namespaces {
    pub fn new() -> Namespaces {
        Namespaces { namespaces: Vec::new() }
    }
    pub fn set(&mut self, prefix: &[u8], namespace: &str) {
        if let Some(ns) = self.namespaces.iter_mut().find(|ns| ns.prefix == prefix) {
            ns.namespace.clear();
            ns.namespace.push_str(namespace);
            return;
        }
        self.namespaces.push(Namespace {
            namespace: String::from(namespace),
            prefix: Vec::from(prefix),
        });
    }
    pub fn insert(&mut self, prefix: &[u8], namespace: String) {
        if let Some(ns) = self.namespaces.iter_mut().find(|ns| ns.prefix == prefix) {
            ns.namespace = namespace;
            return;
        }
        self.namespaces.push(Namespace {
            namespace: namespace,
            prefix: Vec::from(prefix),
        });
    }
    pub fn find_prefix<'a>(&self, iri: &'a str) -> Option<(&[u8], &'a str)> {
        for ns in self.namespaces.iter() {
            if iri.starts_with(&ns.namespace) {
                return Some((ns.prefix.as_slice(), &iri[ns.namespace.len()..]));
            }
        }
        None
    }
    pub fn find_namespace(&self, prefix: &[u8]) -> Option<&str> {
        for ns in self.namespaces.iter() {
            if ns.prefix == prefix {
                return Some(ns.namespace.as_str());
            }
        }
        None
    }
    pub fn iter(&self) -> Iter<Namespace> {
        self.namespaces.iter()
    }
}

#[derive (Clone)]
pub struct Namespace {
    namespace: String,
    prefix: Vec<u8>,
}

impl Namespace {
    pub fn namespace(&self) -> &String {
        &self.namespace
    }
    pub fn prefix(&self) -> &[u8] {
        self.prefix.as_slice()
    }
}
