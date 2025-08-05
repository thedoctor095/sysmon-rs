use anyhow::anyhow as ah;
use chrono::{ DateTime, SecondsFormat, Utc };
use std::time::{ SystemTime, UNIX_EPOCH };
use sysinfo::{ Component, Components, Disk, DiskUsage, Disks, Networks, NetworkData, MINIMUM_CPU_UPDATE_INTERVAL, System};


fn get_host_name() -> Result<String, anyhow::Error> {
    let host_name = System::host_name()
    .ok_or_else(|| ah!("Could not retrieve hostname"))?;
    Ok(host_name)
}

#[derive(Default, Debug)]
pub struct DisksData {
    pub lines: Vec<String>
}

#[derive(Debug)]
pub struct DisksMonitor {
    inner: Disks
}

#[derive(Default, Debug)]
pub struct NetworksData {
    pub lines: Vec<String>
}

#[derive(Debug)]
pub struct NetworksMonitor {
    inner: Networks
}

#[derive(Default, Debug)]
pub struct SensorsData {
    pub lines: Vec<String>
}

#[derive(Debug)]
pub struct SensorsMonitor {
    inner: Components
}


#[derive(Default, Debug)]
pub struct SystemData {
    pub lines: Vec<String>
}

#[derive(Debug)]
pub struct SystemMonitor {
    inner: System
}

pub struct StatsMonitor {
    disks_monitor: DisksMonitor,
    networks_monitor: NetworksMonitor,
    sensors_monitor: SensorsMonitor,
    system_monitor: SystemMonitor
}

#[derive(Default, Debug)]
pub struct SystemStats {
    pub lines: Vec<String>
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
            "name={},mount={},file_system={},kind={},host_name={}", 
            disk_data.name().display().to_string(), 
            disk_data.mount_point().display().to_string(), 
            disk_data.file_system().display().to_string(), 
            disk_data.kind(), get_host_name().expect("Could not retrieve hostname")
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

impl NetworksMonitor {
    pub fn new() -> Self {
        let networks = Networks::new_with_refreshed_list();
        NetworksMonitor { inner: networks }
    }

    pub fn gather_facts(&mut self) -> NetworksData {
        self.inner.refresh(true);
        self.gather().expect("Could not retrieve networks data")
    }

    fn gather(&self) -> Result<NetworksData, anyhow::Error> {
        let mut data = NetworksData::default();
        for (interface, network_data) in &self.inner {
            let tags = self.build_tags(interface, network_data);
            let fields = self.build_fields(network_data);
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
            let data_line = format!("network_usage,{} {} {}", tags, fields, timestamp);
            data.lines.push(data_line);
        };
        Ok(data)
    }

    fn build_tags(&self, interface: &String, networks_data: &NetworkData) -> String {
        let tags = format!(
            "interface={},mac_address={},host_name={}",
            interface.to_string(), networks_data.mac_address().to_string(),
            get_host_name().expect("Could not retrieve hostname")
        );
        tags
    }

    fn build_fields(&self, networks_data: &NetworkData) -> String {
        let fields = format!(
            "received={}i,total_received={}i,transmitted={}i,\
            total_transmitted={}i,packets_received={}i,total_packets_received={}i,\
            packets_transmitted={}i,total_packets_transmitted={}i,\
            errors_on_received={}i,total_errors_on_received={}i,\
            errors_on_transmitted={}i,total_errors_on_transmitted={}i",
            networks_data.received(), networks_data.total_received(),
            networks_data.transmitted(), networks_data.total_transmitted(),
            networks_data.packets_received(), networks_data.total_packets_received(),
            networks_data.packets_transmitted(), networks_data.total_packets_transmitted(),
            networks_data.errors_on_received(), networks_data.total_errors_on_received(),
            networks_data.errors_on_transmitted(), networks_data.total_errors_on_transmitted()
        );
        fields
    }
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
            "sensor_label={},host_name={}",
            sensor_data.label().replace(" ", r"\ "),
            get_host_name().expect("Could not retrieve hostname")
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
        .ok_or_else(|| ah!("Could not convert unix boot time to datetime"))?
        .to_rfc3339_opts(SecondsFormat::Secs, true);
        let tags = format!(
            "boot_time={},host_name={}",
            boot_time, get_host_name().expect("Could not retrieve hostname")
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

impl StatsMonitor {
    pub fn new() -> Self {
        Self {
            disks_monitor: DisksMonitor::new(),
            networks_monitor: NetworksMonitor::new(),
            sensors_monitor: SensorsMonitor::new(),
            system_monitor: SystemMonitor::new()
        }
    }

    pub fn gather_facts(&mut self) -> SystemStats {
        let mut stats = SystemStats::default();

        stats.lines.extend(self.disks_monitor.gather_facts().lines);
        stats.lines.extend(self.networks_monitor.gather_facts().lines);
        stats.lines.extend(self.sensors_monitor.gather_facts().lines);
        stats.lines.extend(self.system_monitor.gather_facts().lines);
        stats
    }
}
