// src/data_storage/mod.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DataPoint {
    pub timestamp: u64,
    pub value: f64,
}

pub struct DataStorage {
    pub storage: HashMap<String, Vec<DataPoint>>,
}

impl DataStorage {
    pub fn new() -> Self {
        DataStorage {
            storage: HashMap::new(),
        }
    }

    pub fn add_data(&mut self, user_id: String, data: DataPoint) {
        self.storage.entry(user_id).or_insert(vec![]).push(data);
    }

    pub fn get_data(&self, user_id: &String) -> Option<&Vec<DataPoint>> {
        self.storage.get(user_id)
    }
}
