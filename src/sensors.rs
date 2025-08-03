use std::time::{ SystemTime, UNIX_EPOCH };
use sysinfo::{ Component, Components };

#[derive(Default, Debug)]
pub struct SensorsData {
    pub lines: Vec<String>
}

#[derive(Debug)]
pub struct SensorsMonitor {
    inner: Components
}

impl SensorsMonitor {
    pub fn new() -> Self {
        let components = Components::new_with_refreshed_list();
        SensorsMonitor { inner: components }
    }

    pub fn gather_facts(&mut self) -> SensorsData {
        self.inner.refresh(true);
        self.gather().expect("Could not retrieve sensors data")
    }

    fn gather(&self) -> Result<SensorsData, anyhow::Error> {
        let mut data = SensorsData::default();
        for component in &self.inner {
            let tags = self.build_tags(component);
            let fields = self.build_fields(component);
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
            let data_line = format!("sensor_data,{} {} {}", tags, fields, timestamp);
            data.lines.push(data_line);
        }
        Ok(data)
    }

    fn build_tags(&self, sensor_data: &Component) -> String {
        format!(
            "sensor_label={}",
            sensor_data.label().replace(" ", r"\ ")
        )
    }

    fn build_fields(&self, sensor_data: &Component) -> String {
        let fields = format!(
            "temperature={},max_temperature={},critical_temperature={}",
            sensor_data.temperature().unwrap_or(0.0), sensor_data.max().unwrap_or(0.0), 
            sensor_data.critical().unwrap_or(0.0)
        );
        fields
    }
}