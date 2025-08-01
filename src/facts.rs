use crate::{
    disks::DisksMonitor, networks::NetworksMonitor, 
    sensors::SensorsMonitor, system::SystemMonitor
};

#[derive(Debug)]
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