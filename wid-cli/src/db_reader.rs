use chrono::Local;
use serde_json::{json, Value};
use std::{fs::File, io::Read};

fn map_value(value: u32, from_min: u32, from_max: u32, to_min: u32, to_max: u32) -> u32 {
    to_min + (value - from_min) * (to_max - to_min) / (from_max - from_min)
}

#[allow(unused)]
pub fn from_date(date: &str, loc: &str) -> Value {
    let mut file = File::open(loc).expect("Cannot find the location of db.json.\nAre you sure the db.json is in the true location or wid-daemon running?");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Unable to read file db.json");
    let mut json_data: Value = serde_json::from_str(&data).unwrap_or_else(|_| json!({}));
    json_data[date].take()
}

#[allow(unused)]
pub fn from_today(loc: &str) -> Value {
    let mut file = File::open(loc).expect("Cannot find the location of db.json.\nAre you sure the db.json is in the true location or wid-daemon running?");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Unable to read file db.json");
    let mut json_data: Value = serde_json::from_str(&data).unwrap_or_else(|_| json!({}));
    let local_time = Local::now();
    let formatted_date = local_time.format("%Y-%m-%d").to_string();
    json_data[formatted_date].take()
}

pub fn analyze_to_v(val: Value) -> Vec<(String, u8)> {
    let mut apps: Vec<(String, u8)> = Vec::new();
    if let Some(array) = val.as_array() {
        for item in array {
            if let Some(apps_array) = item.get("apps").and_then(|v| v.as_array()) {
                for app in apps_array {
                    if let Some(app_str) = app.as_str() {
                        if let Some(existing_app) = apps.iter_mut().find(|(name, _)| name == app_str) {
                            existing_app.1 += 1;
                        } else {
                            apps.push((app_str.to_string(), 1));
                        }
                    }
                }
            }
        }
    }
    apps.iter().map(|(name, time)| (name.clone(), map_value(*time as u32, 0, 60, 0, 100) as u8)).collect()

}
