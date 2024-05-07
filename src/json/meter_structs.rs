use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
#[allow(non_snake_case)]
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
    pub Current_AC_Phase_1: f64,
    pub Current_AC_Phase_2: f64,
    pub Current_AC_Phase_3: f64,
    pub Current_AC_Sum: f64,
    pub Details: Details,
    pub Enable: i32,
    pub EnergyReactive_VArAC_Sum_Consumed: f64,
    pub EnergyReactive_VArAC_Sum_Produced: f64,
    pub EnergyReal_WAC_Minus_Absolute: f64,
    pub EnergyReal_WAC_Plus_Absolute: f64,
    pub EnergyReal_WAC_Sum_Consumed: f64,
    pub EnergyReal_WAC_Sum_Produced: f64,
    pub Frequency_Phase_Average: f64,
    pub Meter_Location_Current: f64,
    pub PowerApparent_S_Phase_1: f64,
    pub PowerApparent_S_Phase_2: f64,
    pub PowerApparent_S_Phase_3: f64,
    pub PowerApparent_S_Sum: f64,
    pub PowerFactor_Phase_1: f64,
    pub PowerFactor_Phase_2: f64,
    pub PowerFactor_Phase_3: f64,
    pub PowerFactor_Sum: f64,
    pub PowerReactive_Q_Phase_1: f64,
    pub PowerReactive_Q_Phase_2: f64,
    pub PowerReactive_Q_Phase_3: f64,
    pub PowerReactive_Q_Sum: f64,
    pub PowerReal_P_Phase_1: f64,
    pub PowerReal_P_Phase_2: f64,
    pub PowerReal_P_Phase_3: f64,
    pub PowerReal_P_Sum: f64,
    pub TimeStamp: i64,
    pub Visible: i64,
    pub Voltage_AC_PhaseToPhase_12: f64,
    pub Voltage_AC_PhaseToPhase_23: f64,
    pub Voltage_AC_PhaseToPhase_31: f64,
    pub Voltage_AC_Phase_1: f64,
    pub Voltage_AC_Phase_2: f64,
    pub Voltage_AC_Phase_3: f64,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Default)]
pub struct Details {
    pub Manufacturer: String,
    pub Model: String,
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
    pub Scope: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Default)]
pub struct Status {
    pub Code: i32,
    pub Reason: String,
    pub UserMessage: String,
}