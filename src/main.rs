use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::time::Duration;

use reqwest::Error;
use tokio::time::sleep;
use url::Url;

use crate::json::{config_structs, meter_structs, powerflow_structs, storage_structs};

mod json {
    pub mod inverter_structs;
    pub mod storage_structs;
    pub mod powerflow_structs;
    pub mod active_device_structs;
    pub mod inverter_info_structs;
    pub mod meter_structs;

    pub mod config_structs;
}

mod power_data_collector;
mod influx_data_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut file = File::open("/opt/fronius-power-monitor/config/fronius-power-monitor.config").expect("Please configure the fronius power monitor first");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("unable to read file");

    let config: config_structs::Config = toml::from_str(&contents).expect("unable to parse config");

    let url_inverter_string = config.power_meter.ip_address;
    let url_influx_string = config.influxdb.host;
    let org = config.influxdb.org;
    let token = config.influxdb.token;
    let bucket = config.influxdb.bucket;
    let poll_timeout = config.power_meter.poll_timeout;

    let url_inverter = Url::from_str(&*url_inverter_string).unwrap();
    let url_influx = Url::from_str(&*url_influx_string).unwrap();

    loop {
        let mut powerflow = powerflow_structs::Root::default();
        let mut storage = storage_structs::Root::default();
        let mut meter = meter_structs::Root::default();

        power_data_collector::get_power_data(&url_inverter, &mut powerflow, &mut storage, &mut meter).await;

        let result = influx_data_handler::push_power_data(storage, powerflow, meter, &org, &token, &bucket, &url_influx).await;
        if result.is_err() { println!("ERROR!! Write to influx failed: {}", result.err().unwrap()) }
        sleep(Duration::from_secs(poll_timeout)).await;
    }
}



