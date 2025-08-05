mod config;
mod influxdb;
mod monitors;

use std::{ thread::sleep, time::Duration };

use crate::{config::Config, influxdb::InfluxDB, monitors::StatsMonitor};


fn main() {
    let config = Config::new();
    let mut client = InfluxDB::default()
    .build(
        &config.influxdb_bucket,
        &config.influxdb_org,
        &config.influxdb_token,
        &config.influxdb_uri
    );

    let mut monitor = StatsMonitor::new();
    run_forever(config, &mut client, &mut monitor);
}

fn run_forever(config: Config, client: &mut InfluxDB, monitor: &mut StatsMonitor) {
    println!(
        "Initialized InfluxDB client - dumping data in db each {:?} seconds",
        config.dump_interval_seconds
    );

    loop {
        let payload = monitor.gather_facts().lines.join("\n");
        if let Some(value) = client.send(payload).err() {
            eprintln!("{value:?}")
        }
        sleep(
            Duration::from_secs(config.dump_interval_seconds)
        );
    }
}
