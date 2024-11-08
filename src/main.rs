use std::io::{Read, Write};
use std::{process::Command, fs::File};
use std::{thread, time};
use chrono::Local;
use serde_json::{json, Value};

fn main() {
    loop {
        control(); // Call your function
        thread::sleep(time::Duration::from_secs(60)); // Wait for 1 minute (60 seconds)
    }
    
}


fn control() {
    let status = Command::new("wmctrl")
        .arg("-l")
        .output()
        .expect("Command failed to start");

    let processes = {
        let mut prc = Vec::new();
        let stdout = String::from_utf8_lossy(&status.stdout); 
        for i in stdout.lines() {
            let true_words: Vec<String> = i.split_whitespace()
                .skip(3)
                .map(|s| s.to_string())
                .collect();     
            let combined = true_words.join(" ");
            prc.push(combined);
        }
        prc
    };
    println!("{:?}", processes);
    println!("Function executed at {:?}", time::SystemTime::now());
    write_to_database(processes);
}

fn write_to_database(strin: Vec<String>) {

    let mut file = File::open("db.json")?;
    let json_data: Value = serde_json::to_value(&strin).unwrap();

    println!("JSON Array: {}", json_data);

    let local_time = Local::now();
    let local_clock = local_time.format("%H %M %S");

    let json_object = json!({
        "apps": strin,
        "time": local_clock.to_string(),
    });
    
}