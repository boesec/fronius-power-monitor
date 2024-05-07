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
    pub DAY_ENERGY: DayEnergy,
    pub PAC: Pac,
    pub TOTAL_ENERGY: TotalEnergy,
    pub YEAR_ENERGY: YearEnergy,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Default)]
pub struct DayEnergy {
    pub Unit: String,
    pub Values: HashMap<String, Option<f64>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Default)]
pub struct Pac {
    pub Unit: String,
    pub Values: HashMap<String, f64>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Default)]
pub struct TotalEnergy {
    pub Unit: String,
    pub Values: HashMap<String, f64>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Default)]
pub struct YearEnergy {
    pub Unit: String,
    pub Values: HashMap<String, Option<f64>>,
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
    pub Scope: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Default)]
pub struct Status {
    pub Code: i32,
    pub Reason: String,
    pub UserMessage: String,
}