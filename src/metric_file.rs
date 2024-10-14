use rust_extensions::date_time::{HourKey, IntervalKey};

#[derive(Debug)]
pub struct MetricFile {
    file_name: String,
    file_size: u64,
}

impl MetricFile {
    pub fn new(file_name: String, file_size: u64) -> Self {
        Self {
            file_name,
            file_size,
        }
    }
    pub fn get_hour_key(&self) -> Option<IntervalKey<HourKey>> {
        if self.file_name.len() < 18 {
            return None;
        }

        let file_index = &self.file_name[8..18];

        let result = file_index.parse::<i64>().ok()?;
        Some(result.into())
    }

    pub fn get_file_size(&self) -> u64 {
        self.file_size
    }
}
