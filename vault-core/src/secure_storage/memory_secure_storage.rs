use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockReadGuard},
};

use super::SecureStorage;

#[derive(Debug)]
pub struct MemorySecureStorage {
    data: Arc<RwLock<HashMap<String, String>>>,
}

impl MemorySecureStorage {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_data(&self) -> RwLockReadGuard<'_, HashMap<String, String>> {
        self.data.read().unwrap()
    }
}

impl SecureStorage for MemorySecureStorage {
    fn get_item(&self, key: &str) -> Result<Option<String>, String> {
        Ok(self.data.read().unwrap().get(key).cloned())
    }

    fn set_item(&self, key: &str, value: &str) -> Result<(), String> {
        self.data
            .write()
            .unwrap()
            .insert(key.to_owned(), value.to_owned());

        Ok(())
    }

    fn remove_item(&self, key: &str) -> Result<(), String> {
        self.data.write().unwrap().remove(key);

        Ok(())
    }
}
