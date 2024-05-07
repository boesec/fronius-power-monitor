use std::collections::HashMap;

use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Default)]
pub struct Root {
    pub Body: Body,
    pub Head: Head,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Default)]
pub struct Body {
    pub Data: Data,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Default)]
pub struct Data {
    pub Inverter: HashMap<String, Device>,
    pub Meter: HashMap<String, Device>,
    pub Ohmpilot: HashMap<String, Device>,
    pub Storage: HashMap<String, Device>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Default)]
pub struct Device {
    pub DT: i32,
    pub Serial: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Default)]
pub struct Head {
    pub RequestArguments: RequestArguments,
    pub Status: Status,
    pub Timestamp: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Default)]
pub struct RequestArguments {
    pub DeviceClass: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Default)]
pub struct Status {
    pub Code: i32,
    pub Reason: String,
    pub UserMessage: String,
}