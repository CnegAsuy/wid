use std::io::{self, BufRead, Read, Write};
use std::{fs::File, thread, time};
use chrono::Local;
use serde_json::{json, Value};
use sysinfo::{ProcessExt, System, SystemExt};

fn main() {
    loop {
        control();
        thread::sleep(time::Duration::from_secs(60));
    }
}

fn control() {

    let mut system = System::new_all();
    system.refresh_all();
    let apps: Vec<String> = apps_tracked();
    let mut processes: Vec<String> = Vec::new();
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

fn apps_tracked() -> Vec<String> {
    let file = File::open("track.txt").expect("Cannot find \"$USER/.cache/wid/track.txt\"");
    let reader = io::BufReader::new(file);
    let mut lines_vec: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines_vec.push(line
            .expect("Can't read the file \"$USER/.cache/wid/track.txt\"")
            .to_lowercase()
            .trim()
            .to_string()
        );
    }

    lines_vec
}