use std::{collections::HashMap, sync::Mutex, time};

pub struct KVData {
    value: String,
    expire_at: Option<time::Instant>,
}

impl KVData {
    pub fn new(value: String, expire_at: Option<time::Instant>) -> Self {
        KVData { value, expire_at }
    }
}

pub struct Store {
    kv: Mutex<HashMap<String, KVData>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            kv: Mutex::new(HashMap::new()),
        }
    }

    pub fn set_kv(&self, key: String, value: KVData) {
        let mut kv = self.kv.lock().unwrap();
        kv.insert(key, value);
    }

    pub fn get_kv(&self, key: String) -> Option<String> {
        let mut kv = self.kv.lock().unwrap();

        let data = kv.get(&key)?;

        match data.expire_at {
            Some(expire_at) if time::Instant::now() > expire_at => {
                kv.remove(&key);
                return None;
            }
            _ => Some(data.value.clone()),
        }
    }
}
