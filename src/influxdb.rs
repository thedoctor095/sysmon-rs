use anyhow;
use reqwest::header::{ HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_TYPE };
use std::time::Duration;
use url::Url;

const CTYPE: &str = "text/plain; charset=utf-8";
const ACC: &str = "application/json";

#[derive(Debug, Default)]
pub struct InfluxDB {
    headers: HeaderMap,
    influxdb_token: String,
    pub influxdb_uri: String
}

impl InfluxDB {
    pub fn build(mut self, bucket: &str, org: &str, token: &str, uri: &str) -> Self {
        let mut url = Url::parse(uri)
        .expect("Invalid InfluxDB URI found");

        url
        .query_pairs_mut()
        .append_pair("bucket", bucket)
        .append_pair("org", org)
        .append_pair("precision", "ms");

        self.influxdb_uri = url.to_string();
        self.influxdb_token = token.to_string();
        self
    }

    pub fn send(&mut self, payload: String) -> Result<(), anyhow::Error>{
        let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(60))
        .build()
        .expect("Could not build HTTP client");

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