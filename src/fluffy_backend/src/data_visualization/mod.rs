use crate::data_storage::DataPoint;

pub struct DataVisualizer;

impl DataVisualizer {
    pub fn format_data_for_chart(data: &[DataPoint]) -> Vec<(u64, f64)> {
        data.iter().map(|d| (d.timestamp, d.value)).collect()
    }
}
