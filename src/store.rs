use dashmap::DashMap;
use std::{ops::Deref, sync::Arc};

use crate::resp::frame::Frame;

#[derive(Debug, Clone)]
pub struct Store {
    inner: Arc<StoreInner>,
}

#[derive(Debug)]
pub struct StoreInner {
    map: DashMap<String, Frame>,
    hmap: DashMap<String, DashMap<String, Frame>>,
}

impl Default for Store {
    fn default() -> Self {
        let inner = Arc::new(StoreInner::default());
        Self { inner }
    }
}

impl Default for StoreInner {
    fn default() -> Self {
        Self {
            map: DashMap::new(),
            hmap: DashMap::new(),
        }
    }
}

impl Deref for Store {
    type Target = StoreInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Store {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &str) -> Option<Frame> {
        self.map.get(key).map(|v| v.value().clone())
    }

    pub fn set(&self, key: impl ToString, value: Frame) {
        self.map.insert(key.to_string(), value);
    }

    pub fn hset(&self, key: impl ToString, field: impl ToString, value: Frame) {
        let hmap = self.hmap.entry(key.to_string()).or_default();
        hmap.insert(field.to_string(), value);
    }

    pub fn hget(&self, key: &str, field: &str) -> Option<Frame> {
        self.hmap
            .get(key)
            .and_then(|hmap| hmap.get(field).map(|v| v.value().clone()))
    }

    pub fn hgetall(&self, key: &str) -> Option<DashMap<String, Frame>> {
        self.hmap.get(key).map(|v| v.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_get_set() {
        let store = Store::new();
        store.set("key", "value".into());
        let result = store.get("key").unwrap();
        assert_eq!(result, "value".into());
    }

    #[test]
    fn test_store_hset_hget() {
        let store = Store::new();
        store.hset("key", "field", "value".into());
        let result = store.hget("key", "field").unwrap();
        assert_eq!(result, "value".into());
    }

    #[test]
    fn test_store_hgetall() {
        let store = Store::new();
        store.hset("key", "field1", "value1".into());
        store.hset("key", "field2", "value2".into());
        let result = store.hgetall("key").unwrap();
        assert_eq!(result.len(), 2);
    }
}
