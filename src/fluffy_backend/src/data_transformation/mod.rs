// src/data_transformation/mod.rs
use crate::data_storage::DataPoint;

pub struct DataTransformer;

impl DataTransformer {
    pub fn average(data: &[DataPoint]) -> Option<f64> {
        let sum: f64 = data.iter().map(|point| point.value).sum();
        let count = data.len();
        if count > 0 {
            Some(sum / count as f64)
        } else {
            None
        }
    }
}
