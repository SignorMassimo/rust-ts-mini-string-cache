use std::collections::HashMap;

use napi::{Error, Result, Status};
use napi_derive::napi;

#[napi(object)]
pub struct CacheData {
    pub key: String,
    pub value: String,
}

#[napi]
pub struct MiniCache {
    store: HashMap<String, String>,
}

#[napi]
impl MiniCache {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    #[napi]
    pub fn add(&mut self, key: String, value: String) -> Result<bool> {
        if self.store.contains_key(&key) {
            return Err(Error::new(
                Status::InvalidArg,
                format!("'{}' key already exists", key),
            ));
        }
        self.store.insert(key, value);
        Ok(true)
    }

    #[napi]
    pub fn set(&mut self, key: String, value: String) -> Result<bool> {
        match self.store.get_mut(&key) {
            Some(v) => {
                *v = value;
                Ok(true)
            }
            None => Err(Error::new(
                Status::InvalidArg,
                format!("unknown key '{}'", key),
            )),
        }
    }

    #[napi]
    pub fn get(&self, key: String) -> Result<String> {
        match self.store.get(&key) {
            Some(v) => Ok(v.clone()),
            None => Err(Error::new(
                Status::InvalidArg,
                format!("unknown key '{}'", key),
            )),
        }
    }

    #[napi]
    pub fn filter(&self, keys: Vec<String>) -> Vec<CacheData> {
        let filtered = self
            .store
            .iter()
            .map(|(k, v)| {
                if keys.contains(k) {
                    CacheData {
                        key: k.clone(),
                        value: v.clone(),
                    }
                } else {
                    CacheData {
                        key: "".to_string(),
                        value: "".to_string(),
                    }
                }
            })
            .collect();
        filtered
    }

    #[napi]
    pub fn has(&self, key: String) -> bool {
        self.store.contains_key(&key)
    }

    #[napi]
    pub fn delete(&mut self, key: String) -> Result<bool> {
        match self.store.get(&key) {
            Some(_) => {
                self.store.remove(&key);
                Ok(true)
            }
            None => Err(Error::new(
                Status::InvalidArg,
                format!("'{}' unknown key.", key),
            )),
        }
    }

    #[napi]
    pub fn all(&self) -> Vec<CacheData> {
        self.store
            .iter()
            .map(|(k, v)| CacheData {
                key: k.clone(),
                value: v.clone(),
            })
            .collect()
    }
}
