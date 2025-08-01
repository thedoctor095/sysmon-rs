use std::time::{ SystemTime, UNIX_EPOCH };
use sysinfo::{ Networks, NetworkData };


#[derive(Default, Debug)]
pub struct NetworksData {
    pub lines: Vec<String>
}

#[derive(Debug)]
pub struct NetworksMonitor {
    inner: Networks
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
            "interface={},mac_address={}",
            interface.to_string(), networks_data.mac_address().to_string()
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