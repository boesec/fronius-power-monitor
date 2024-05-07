use url::Url;

use crate::json::{meter_structs, powerflow_structs};
use crate::json::storage_structs;

pub async fn get_power_data(host: &Url, powerflow_data: &mut powerflow_structs::Root, storage_data: &mut storage_structs::Root, meter_realtime_data: &mut meter_structs::Root) {
    let storage_request_data: Option<storage_structs::Root> = get_storage_data(host).await;
    let power_flow_request_data: Option<powerflow_structs::Root> = get_power_flow_data(host).await;
    let meter_realtime_request_data: Option<meter_structs::Root> = get_meter_realtime_data(host).await;

    match power_flow_request_data {
        Some(data) => {
            *powerflow_data = data;
        }
        None => {
            *powerflow_data = powerflow_structs::Root::default();
        }
    }

    match storage_request_data {
        Some(data) => {
            *storage_data = data;
        }
        None => {
            *storage_data = storage_structs::Root::default();
        }
    }

    match meter_realtime_request_data {
        Some(data) => {
            *meter_realtime_data = data;
        }
        None => {
            *meter_realtime_data = meter_structs::Root::default();
        }
    }
}

async fn get_power_flow_data(host: &Url) -> Option<powerflow_structs::Root> {
    let url = format!("{}solar_api/v1/GetPowerFlowRealtimeData.fcgi", host);
    let response = reqwest::get(&url).await;

    return match response {
        Ok(res) => {
            if res.status().is_success() {
                // Parse the response body as JSON
                let data: Result<powerflow_structs::Root, _> = res.json().await;
                match data {
                    Ok(da) => {
                        Some(da)
                    }
                    _ => { None }
                }
            } else {
                println!("Request failed with status code {}", res.status());
                None
            }
        }
        _ => { None }
    };
}

async fn get_meter_realtime_data(host: &Url) -> Option<meter_structs::Root> {
    let url = format!("{}solar_api/v1/GetMeterRealtimeData.cgi", host);
    let response = reqwest::get(&url).await;

    return match response {
        Ok(res) => {
            if res.status().is_success() {
                // Parse the response body as JSON
                let data: Result<meter_structs::Root, _> = res.json().await;
                match data {
                    Ok(da) => {
                        Some(da)
                    }
                    _ => { None }
                }
            } else {
                println!("Request failed with status code {}", res.status());
                None
            }
        }
        _ => { None }
    };
}

async fn get_storage_data(host: &Url) -> Option<storage_structs::Root> {
    let url = format!("{}solar_api/v1/GetStorageRealtimeData.cgi", host);
    let response = reqwest::get(&url).await;

    return match response {
        Ok(res) => {
            if res.status().is_success() {
                // Parse the response body as JSON
                let data: Result<storage_structs::Root, _> = res.json().await;
                match data {
                    Ok(da) => {
                        Some(da)
                    }
                    _ => { None }
                }
            } else {
                println!("Request failed with status code {}", res.status());
                None
            }
        }
        _ => { None }
    };
}
