use anyhow;
use configparser::ini::Ini;
use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use std::{env::home_dir};
use url::Url;

const CTYPE: &str = "text/plain; charset=utf-8";
const ACC: &str = "application/json";

#[derive(Debug)]
pub struct InfluxDB {
    pub dump_interval_seconds: u64,
    headers: HeaderMap,
    influxdb_token: String,
    pub influxdb_uri: String
}

impl InfluxDB {
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
        
        let mut url = Url::parse(&influxdb_uri)
        .expect("Invalid InfluxDB URI found");

        url
        .query_pairs_mut()
        .append_pair("bucket", &influxdb_bucket)
        .append_pair("org", &influxdb_org)
        .append_pair("precision", "ms");
        
        Self {
            dump_interval_seconds: dump_interval,
            influxdb_token: influxdb_token,
            influxdb_uri: url.to_string(),
            headers: HeaderMap::default()
        }
    }

    pub fn send(&mut self, payload: String) -> Result<(), anyhow::Error>{
        let client = reqwest::blocking::Client::new();
        if self.headers.is_empty() {
            self.build_headers();
        }
        let response = client
        .post(&self.influxdb_uri)
        .body(payload)
        .headers(self.headers.clone())
        .send()?;
        println!("[InfluxDB] <{} {}>", response.status(), response.url());
        Ok(())
    }

    fn build_headers(&mut self) {
        self.headers.insert(
            CONTENT_TYPE, CTYPE
            .parse()
            .expect("Could not build header CONTENT_TYPE")
        );
        self.headers.insert(
            ACCEPT, ACC
            .parse()
            .expect("Could not build header ACCEPT")
        );

        self.headers.insert(
            AUTHORIZATION, format!("Token {}", &self.influxdb_token)
            .parse()
            .expect("Could not build header AUTHORIZATION")
        );
    }
}