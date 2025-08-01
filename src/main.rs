mod influxdb;
mod disks;
mod facts;
mod networks;
mod sensors;
mod system;

use std::time::Duration;

use crate::{influxdb::InfluxDB, facts::StatsMonitor};


fn main() {
    let mut client = InfluxDB::new();
    let mut monitor = StatsMonitor::new();
    println!(
        "Initialized InfluxDB client for {:?} - dumping data in db each {:?} seconds", 
        client.influxdb_uri, client.dump_interval_seconds
    );
    
    loop {
        let payload = monitor.gather_facts().lines.join("\n");
        if let Some(value) = client.send(payload).err() {
            eprintln!("{value:?}")
        }
        std::thread::sleep(Duration::from_secs(client.dump_interval_seconds));
    }
}
