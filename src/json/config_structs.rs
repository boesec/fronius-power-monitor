use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub(crate) power_meter: PowerMeter,
    pub(crate) influxdb: InfluxDB,
}

#[derive(Deserialize)]
pub(crate) struct PowerMeter {
    pub(crate) ip_address: String,
    pub(crate) poll_timeout: u64,
}

#[derive(Deserialize)]
pub(crate) struct InfluxDB {
    pub(crate) host: String,
    pub(crate) bucket: String,
    pub(crate) token: String,
    pub(crate) org: String,
}