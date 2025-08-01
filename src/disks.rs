use anyhow;
use std::time::{ SystemTime, UNIX_EPOCH };
use sysinfo::{Disk, DiskUsage, Disks};

#[derive(Default, Debug)]
pub struct DisksData {
    pub lines: Vec<String>
}

#[derive(Debug)]
pub struct DisksMonitor {
    inner: Disks
}

impl DisksMonitor {
    pub fn new() -> Self {
        let disks = Disks::new_with_refreshed_list();
        DisksMonitor { inner: disks }
    }

    pub fn gather_facts(&mut self) -> DisksData {
        self.inner.refresh(true);
        self.gather().expect("Could not retrieve disks data")
    }

    fn gather(&self) -> Result<DisksData, anyhow::Error> {
        let mut data = DisksData::default();
        for disk in &self.inner {
            let usage = disk.usage();
            let tags = self.build_tags(&disk);
            let fields = self.build_fields(&disk, &usage);
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
            let data_line = format!("disk_usage,{} {} {}", tags, fields, timestamp);
            data.lines.push(data_line);
        };
        Ok(data)
    }

    fn build_tags(&self, disk_data: &Disk) -> String {
        let tags = format!(
            "name={},mount={},file_system={},kind={}", 
            disk_data.name().display().to_string(), 
            disk_data.mount_point().display().to_string(), 
            disk_data.file_system().display().to_string(), 
            disk_data.kind()
        );
        tags
    }

    fn build_fields(&self, disk_data: &Disk, disk_usage: &DiskUsage) -> String {
        let fields = format!(
            "available_space={}i,total_space={}i,total_written={}i,\
            written_since={}i,total_read={}i,read_since={}i",
            disk_data.available_space(),
            disk_data.total_space(),
            disk_usage.total_written_bytes,
            disk_usage.written_bytes,
            disk_usage.total_read_bytes,
            disk_usage.read_bytes
        );
        fields
    }


}