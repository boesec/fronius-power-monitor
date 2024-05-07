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
    pub Data: HashMap<String, Data>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Default)]
pub struct Data {
    pub CustomName: String,
    pub DT: i32,
    pub ErrorCode: i32,
    pub InverterState: String,
    pub PVPower: i32,
    pub Show: i32,
    pub StatusCode: i32,
    pub UniqueID: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Default)]
pub struct Head {
    pub RequestArguments: HashMap<String, String>,
    pub Status: Status,
    pub Timestamp: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Default)]
pub struct Status {
    pub Code: i32,
    pub Reason: String,
    pub UserMessage: String,
}