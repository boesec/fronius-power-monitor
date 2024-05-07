use futures::prelude::*;
use influxdb2::Client;
use influxdb2::models::DataPoint;
use url::Url;

use crate::json::{meter_structs, powerflow_structs, storage_structs};

pub async fn push_power_data(storage_data: storage_structs::Root, powerflow_data: powerflow_structs::Root, meter_data: meter_structs::Root,
                             org: &String, token: &String, bucket: &String, host: &Url) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(host.to_string(), org, token);
    let inverters = &powerflow_data.Body.Data.Inverters.get("1").unwrap().clone(); //TODO add dynamic data point creation based on inverters array
    let site = powerflow_data.Body.Data.Site;
    let storage = storage_data.Body.Data.get("0").unwrap().Controller.clone(); //TODO add dynamic data point creation based on controller array
    let meter_details = &meter_data.Body.Data.get("0").unwrap().Details;

    let points = vec![
        DataPoint::builder("inverter")
            .tag("Manufacturer", &meter_details.Manufacturer)
            .tag("Model", &meter_details.Model)
            .tag("Serial", &meter_details.Serial)
            .tag("number", "1")
            .tag("battery_mode", &inverters.Battery_Mode)
            .field("dt", inverters.DT as i64)
            .field("e_total", inverters.E_Total)
            .field("p", inverters.P)
            .field("soc", inverters.SOC)
            .build()?,
        DataPoint::builder("site")
            .tag("Manufacturer", &meter_details.Manufacturer)
            .tag("Model", &meter_details.Model)
            .tag("Serial", &meter_details.Serial)
            .tag("backup_mode", site.BackupMode.to_string())
            .tag("battery_standby", site.BatteryStandby.to_string())
            .tag("meter_location", site.Meter_Location)
            .tag("mode", site.Mode)
            .field("e_total", inverters.E_Total)
            .field("p_akku", site.P_Akku)
            .field("p_grid", site.P_Grid)
            .field("p_load", site.P_Load)
            .field("p_pv", site.P_PV)
            .field("rel_autonomy", site.rel_Autonomy)
            .field("rel_selfconsumption", site.rel_SelfConsumption.unwrap_or(0.0))
            .build()?,
        DataPoint::builder("storage")
            .tag("Manufacturer", storage.Details.Manufacturer)
            .tag("Model", storage.Details.Model)
            .tag("Serial", storage.Details.Serial.trim())
            .field("capacity_maximum", storage.Capacity_Maximum)
            .field("current_dc", storage.Current_DC)
            .field("designed_capacity", storage.DesignedCapacity)
            .field("enable", storage.Enable as i64)
            .field("state_of_charge_relative", storage.StateOfCharge_Relative)
            .field("status_battery_cell", storage.Status_BatteryCell)
            .field("temperature_cell", storage.Temperature_Cell)
            .field("voltage_dc", storage.Voltage_DC)
            .build()?,
    ];

    client.write(bucket, stream::iter(points)).await?;

    Ok(())
}