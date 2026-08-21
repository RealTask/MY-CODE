//! Project and conversation memory for continuity.

use anyhow::Result;
use parking_lot::Mutex;
use std::collections::HashMap;

/// Key-value memory store used across sessions.
#[derive(Debug, Default)]
pub struct Memory {
    entries: Mutex<HashMap<String, String>>,
}

impl Memory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&self, key: impl Into<String>, value: impl Into<String>) {
        self.entries.lock().insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.entries.lock().get(key).cloned()
    }

    pub fn remove(&self, key: &str) -> Option<String> {
        self.entries.lock().remove(key)
    }

    pub fn clear(&self) {
        self.entries.lock().clear();
    }

    pub fn len(&self) -> usize {
        self.entries.lock().len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.lock().is_empty()
    }

    /// Snapshot of all entries.
    pub fn snapshot(&self) -> Result<HashMap<String, String>> {
        Ok(self.entries.lock().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_get_remove() {
        let mem = Memory::new();
        mem.set("k", "v");
        assert_eq!(mem.get("k").as_deref(), Some("v"));
        assert_eq!(mem.remove("k").as_deref(), Some("v"));
        assert!(mem.is_empty());
    }
}
