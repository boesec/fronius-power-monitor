use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[allow(non_snake_case)]
pub struct Root {
    pub Body: Body,
    pub Head: Head,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Body {
    pub Data: Data,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[allow(non_snake_case)]
pub struct Data {
    pub Inverters: HashMap<String, Inverter>,
    pub SecondaryMeters: HashMap<String, serde_json::Value>,
    pub Site: Site,
    pub Smartloads: Smartloads,
    pub Version: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[allow(non_snake_case)]
pub struct Inverter {
    pub Battery_Mode: String,
    pub DT: i32,
    pub E_Day: Option<f64>,
    pub E_Total: f64,
    pub E_Year: Option<f64>,
    pub P: f64,
    pub SOC: f64,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Site {
    pub BackupMode: bool,
    pub BatteryStandby: bool,
    pub E_Day: Option<f64>,
    pub E_Total: f64,
    pub E_Year: Option<f64>,
    pub Meter_Location: String,
    pub Mode: String,
    pub P_Akku: f64,
    pub P_Grid: f64,
    pub P_Load: f64,
    pub P_PV: f64,
    pub rel_Autonomy: f64,
    pub rel_SelfConsumption: Option<f64>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Smartloads {
    pub Ohmpilots: HashMap<String, serde_json::Value>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Head {
    pub RequestArguments: HashMap<String, serde_json::Value>,
    pub Status: Status,
    pub Timestamp: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Status {
    pub Code: i32,
    pub Reason: String,
    pub UserMessage: String,
}
