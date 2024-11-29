use chrono::Local;
use serde_json::{json, Value};
use std::{fs::File, io::Read};

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



