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
    let _ = write_to_database(processes); 
}


fn write_to_database(strin: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("db.json")?;
    let mut readen_file = String::new();
    file.read_to_string(&mut readen_file)?;
    let mut json_data: Value = serde_json::from_str(&readen_file)?;
    
    let local_time = Local::now();
    let local_clock = local_time.format("%H %M %S");
    
    let json_object = json!({
        "apps": strin,
        "time": local_clock.to_string(),
    });


    let formatted_date = local_time.format("%Y-%m-%d").to_string();
    println!("Formatted Date: {}", formatted_date);

    if let Some(data) = json_data.get_mut(&formatted_date).and_then(|d| d.as_array_mut()) {
        println!("Data found for {}: {:?}", formatted_date, data);
        data.push(json_object.clone());
    } else {
        println!("No data found for {}. Creating new entry.", formatted_date);
        json_data[formatted_date] = json!([json_object]);
    }
    
    let mut file = File::create("db.json")?;
    file.write_all(json_data.to_string().as_bytes())?;

    println!("{:?}", json_data);
    println!("{:?}", json_object);

    json_data.get("index");

    Ok(())
}