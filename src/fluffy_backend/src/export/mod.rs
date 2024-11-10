use crate::data_storage::DataPoint;

pub struct DataExporter;

impl DataExporter {
    pub fn to_csv(data: &[DataPoint]) -> String {
        data.iter()
            .map(|d| format!("{},{}", d.timestamp, d.value))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
