use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Root {
    pub Body: Body,
    pub Head: Head,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Body {
    pub Data: HashMap<String, DataItem>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DataItem {
    pub Controller: Controller,
    pub Modules: Vec<serde_json::Value>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Controller {
    pub Capacity_Maximum: f64,
    pub Current_DC: f64,
    pub DesignedCapacity: f64,
    pub Details: Details,
    pub Enable: i32,
    pub StateOfCharge_Relative: f64,
    pub Status_BatteryCell: f64,
    pub Temperature_Cell: f64,
    pub TimeStamp: i64,
    pub Voltage_DC: f64,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Details {
    pub Manufacturer: String,
    pub Model: String,
    pub Serial: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Head {
    pub RequestArguments: RequestArguments,
    pub Status: Status,
    pub Timestamp: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestArguments {
    pub Scope: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Status {
    pub Code: i32,
    pub Reason: String,
    pub UserMessage: String,
}