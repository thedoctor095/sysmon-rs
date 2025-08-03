use configparser::ini::Ini;
use std::env::home_dir;

#[derive(Debug)]
pub struct Config {
    pub dump_interval_seconds: u64,
    pub influxdb_bucket: String,
    pub influxdb_org: String,
    pub influxdb_token: String,
    pub influxdb_uri: String
}

impl Config {
    pub fn new() -> Self {
        let mut config = Ini::new();

        let config_path = home_dir()
        .expect("Could not infer home dir path - failed to initialize")
        .join(".config/sysmon.ini");

        let _ = config
        .load(config_path)
        .expect("Could not load config path");
    
        // default to hourly dumps
        let dump_interval = config
        .getuint("sys-mon", "dump_interval_seconds")
        .unwrap_or_else(|_| None)
        .unwrap_or(3600);

        let influxdb_bucket = config
        .get("influxdb", "bucket")
        .expect("InfluxDB bucket not found");

        let influxdb_org = config
        .get("influxdb", "organisation")
        .expect("IncludDB organization not found");
    
        let influxdb_token = config
        .get("influxdb", "token")
        .expect("InfluxDB token not found");

        let influxdb_uri = config
        .get("influxdb", "uri")
        .expect("InfluxDB URI not found");

        Self {
            dump_interval_seconds: dump_interval,
            influxdb_bucket: influxdb_bucket,
            influxdb_org: influxdb_org,
            influxdb_token: influxdb_token,
            influxdb_uri: influxdb_uri
        }
    }
}