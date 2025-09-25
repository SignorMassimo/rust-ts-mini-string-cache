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
        if self.store.contains_key(&key.clone()) {
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
        if let Some(v) = self.store.get_mut(&key) {
            *v = value;
            return Ok(true);
        } else {
            return Err(Error::new(
                Status::InvalidArg,
                format!("unknown key '{}'", key),
            ));
        }
    }

    #[napi]
    pub fn get(&self, key: String) -> Result<String> {
        if let Some(v) = self.store.get(&key) {
            return Ok(v.clone());
        }
        return Err(Error::new(
            Status::InvalidArg,
            format!("unknown key '{}'", key),
        ));
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
