//! A K-V Store lib

#![deny(missing_docs)]
use std::collections::HashMap;

/// KvStore store string-string kv pairs
#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    /// create a KvStore
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    /// let store = KvStore::new();
    /// ```
    pub fn new() -> Self {
        KvStore {
            store: HashMap::new(),
        }
    }

    /// set a k-v pair to the KvStore, if the key was existed, then the value will be overwritten
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("key".to_string(), "value".to_string());
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    /// get th value of key, if key not existed, return None
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// assert_eq!(None, store.get("key".to_string()));
    ///
    /// store.set("key".to_string(), "value".to_string());
    /// assert_eq!(Some("value".to_string()), store.get("key".to_string()));
    /// ```
    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).map(|v| v.to_owned())
    }

    /// remove the key and it's values from KvStore
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.remove("key".to_string());
    /// ```
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
