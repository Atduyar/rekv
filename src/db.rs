#![allow(dead_code, unused)]
use crate::kv_value::KVValue;
use std::{collections::HashMap, sync::Mutex};

type Key = String;
type Value = KVValue;

pub struct DB {
    db: Mutex<HashMap<Key, Value>>
}

impl DB {
    pub fn new() -> DB {
        DB { db: Mutex::new(HashMap::new()) }
    }

    pub fn get(&self, key: &Key) -> Option<Value> {
        let mut db = self.db.lock().unwrap();
        db.get(key).cloned()
    }

    pub fn insert(&mut self, key: Key, value: Value) -> Option<Value> {
        let mut db = self.db.lock().unwrap();
        db.insert(key, value)
    }

    pub fn remove(&mut self, key: &Key) -> Option<Value> {
        let mut db = self.db.lock().unwrap();
        db.remove(key)
    }
}
