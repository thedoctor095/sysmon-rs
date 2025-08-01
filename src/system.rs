use anyhow::anyhow as manyhow;
use chrono::{ DateTime, SecondsFormat, Utc };
use std::time::{ SystemTime, UNIX_EPOCH };
use sysinfo::{ MINIMUM_CPU_UPDATE_INTERVAL, System };


#[derive(Default, Debug)]
pub struct SystemData {
    pub lines: Vec<String>
}

#[derive(Debug)]
pub struct SystemMonitor {
    inner: System
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut sys = System::new();
        sys.refresh_all();
        Self { inner: sys }
    }

    pub fn gather_facts(&mut self) -> SystemData {
        self.inner.refresh_cpu_all();
        std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
        self.inner.refresh_cpu_all();
        self.inner.refresh_memory();
        self.gather().expect("Could not retrieve system data")
    }

    fn gather(&self) -> Result<SystemData, anyhow::Error> {
        let mut data = SystemData::default();
        let tags = self.build_tags()?;
        let fields = self.build_fields();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
        let data_line = format!("system_usage,{} {} {}", tags, fields, timestamp);
        data.lines.push(data_line);
        Ok(data)
    }

    fn build_tags(&self) -> Result<String, anyhow::Error> {
        let current_unix = System::boot_time() as i64;
        let boot_time = DateTime::<Utc>::from_timestamp(current_unix, 0)
        .ok_or_else(|| manyhow!("Could not convert unix boot_time to datetime"))?
        .to_rfc3339_opts(SecondsFormat::Secs, true);
        let host_name = System::host_name()
        .ok_or_else(|| manyhow!("Could not retrieve hostname"))?;
        let tags = format!(
            "boot_time={},host_name={}",
            boot_time, host_name
        );
        Ok(tags)
    }

    fn build_fields(&self) -> String {
        let fields = format!(
            "available_memory={}i,free_memory={}i,used_memory={}i,free_swap={}i,\
            used_swap={}i,total_swap={}i,global_cpu_usage={}",
            self.inner.available_memory(), self.inner.free_memory(), self.inner.used_memory(),
            self.inner.free_swap(), self.inner.used_swap(), self.inner.total_swap(),
            self.inner.global_cpu_usage()
        );
        fields
    }
}