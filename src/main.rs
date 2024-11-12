use std::io::{Read, Write};
use std::{process::Command, fs::File};
use std::{thread, time};
use chrono::Local;
use serde_json::{json, Value};

fn main() {
    loop {
        control();
        thread::sleep(time::Duration::from_secs(60));
    }
}

fn control() {
    let status = Command::new("wmctrl")
        .arg("-l")
        .output()
        .expect("Command failed to start");

    let processes: Vec<String> = String::from_utf8_lossy(&status.stdout)
        .lines()
        .filter_map(|line| line.split_whitespace().skip(3).next().map(String::from))
        .collect();
    
    let _ = write_to_database(processes);
}

fn write_to_database(strin: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("db.json").unwrap_or_else(|_| File::create("db.json").unwrap());
    let mut readen_file = String::new();
    file.read_to_string(&mut readen_file)?;

    let mut json_data: Value = serde_json::from_str(&readen_file).unwrap_or_else(|_| json!({}));

    let local_time = Local::now();
    let local_clock = local_time.format("%H:%M").to_string();
    let json_object = json!({
        "apps": strin,
        "time": local_clock,
    });

    let formatted_date = local_time.format("%Y-%m-%d").to_string();

    if let Some(data) = json_data.get_mut(&formatted_date).and_then(|d| d.as_array_mut()) {
        data.push(json_object);
    } else {
        json_data[formatted_date] = json!([json_object]);
    }

    let mut file = File::create("db.json")?;
    file.write_all(json_data.to_string().as_bytes())?;

    Ok(())
}
