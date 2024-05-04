use dashmap::{DashMap, DashSet};
use std::{ops::Deref, sync::Arc};

use crate::resp::frame::Frame;

#[derive(Debug, Clone)]
pub struct Backend {
    inner: Arc<BackendInner>,
}

#[derive(Debug)]
pub struct BackendInner {
    set: DashMap<String, DashSet<String>>,
    map: DashMap<String, Frame>,
    hmap: DashMap<String, DashMap<String, Frame>>,
}

impl Default for Backend {
    fn default() -> Self {
        let inner = Arc::new(BackendInner::default());
        Self { inner }
    }
}

impl Default for BackendInner {
    fn default() -> Self {
        Self {
            set: DashMap::new(),
            map: DashMap::new(),
            hmap: DashMap::new(),
        }
    }
}

impl Deref for Backend {
    type Target = BackendInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Backend {
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

    pub fn sadd(&self, key: &str, field: &str) -> bool {
        let set = self.set.entry(key.to_string()).or_default();

        if set.contains(field) {
            false
        } else {
            set.insert(field.to_string());
            true
        }
    }

    pub fn smembers(&self, key: &str) -> Option<Vec<String>> {
        self.set
            .get(key)
            .map(|v| v.iter().map(|v| v.clone()).collect())
    }

    pub fn sismember(&self, key: &str, field: &str) -> bool {
        self.set.get(key).map_or(false, |v| v.contains(field))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_get_set() {
        let backend = Backend::new();
        backend.set("key", "value".into());
        let result = backend.get("key").unwrap();
        assert_eq!(result, "value".into());
    }

    #[test]
    fn test_backend_hset_hget() {
        let backend = Backend::new();
        backend.hset("key", "field", "value".into());
        let result = backend.hget("key", "field").unwrap();
        assert_eq!(result, "value".into());
    }

    #[test]
    fn test_backend_hgetall() {
        let backend = Backend::new();
        backend.hset("key", "field1", "value1".into());
        backend.hset("key", "field2", "value2".into());
        let result = backend.hgetall("key").unwrap();
        assert_eq!(result.len(), 2);
    }
}
