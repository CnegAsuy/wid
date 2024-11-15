use std::io::{self, BufRead, Read, Write};
use std::{fs::File, thread, time};
use chrono::Local;
use serde_json::{json, Value};
use sysinfo::{ProcessExt, System, SystemExt};

fn main() {
    // Create a loop for check every process and write it on file.
    loop {
        control();
        thread::sleep(time::Duration::from_secs(60));
    }
}

fn control() {
    // Take all the processes
    let mut system = System::new_all();
    system.refresh_all();
    let apps: Vec<String> = apps_tracked();
    let mut processes: Vec<String> = Vec::new();
    // Select the project that i want.
    for (_pid, process) in system.processes() {
        println!("{:?}", process.name());
        if !processes.contains(&process.name()
            .to_lowercase()
            .trim()
            .to_string()) &&
            apps.contains(&process.name().to_lowercase().trim().to_string()) {
            processes.push(process.name().to_lowercase().trim().to_string());
        }
    }
    // Write it on db.json
    let _ = write_to_database(processes);
}

fn write_to_database(strin: Vec<String>) {
    // Open the file if doesn't exist create new one.
    let mut file = File::open("db.json").unwrap_or_else(|_| File::create("db.json").expect("Failed to open file: \"~/.cache/wid/db.json\""));
    let mut readen_file = String::new();
    file.read_to_string(&mut readen_file).expect("unable to read file: \"~/.cache/wid/db.json\"");
    let mut json_data: Value = serde_json::from_str(&readen_file).unwrap_or_else(|_| json!({}));
    let local_time = Local::now();
    let local_clock = local_time.format("%H:%M").to_string();
    // Prepare the json object that i will write on this file.
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

    // Write the data on db.json.
    let mut file = File::create("db.json").expect("failed to create file: \"~/.cache/wid/db.json\"");
    file.write_all(json_data.to_string().as_bytes()).expect("failed to write data on file: \"~/.cache/wid/db.json\"");
}

fn apps_tracked() -> Vec<String> {
    // Read the file for track files.
    let file = File::open("track.txt").unwrap_or_else(|_| File::create("track.txt").expect("Failed to open file: \"~/.cache/wid/track.txt\""));
    let reader = io::BufReader::new(file);
    let mut lines_vec: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines_vec.push(line
            .expect("Cannot read file: \"~/.cache/wid/track.txt\"")
            .to_lowercase()
            .trim()
            .to_string()
        );
    }
    lines_vec
}