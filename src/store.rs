use std::collections::HashMap;

pub struct Store {
    kv: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Store {
        Store { kv: HashMap::new() }
    }

    pub fn set_kv(&mut self, key: String, value: String) {
        self.kv.insert(key, value);
    }

    pub fn get_kv(&self, key: String) -> Option<String> {
        self.kv.get(&key).cloned()
    }
}
