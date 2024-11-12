use std::io::{Read, Write};
use std::{fs::File, env, thread, time};
use chrono::Local;
use serde_json::{json, Value};
use sysinfo::{ProcessExt, System, SystemExt, UserExt};

fn main() {
    loop {
        control();
        thread::sleep(time::Duration::from_secs(60));
    }
}

fn control() {

    let mut system = System::new_all();
    system.refresh_all();

    let current_user_uid = system.users().iter()
        .find(|user| user.name() == env::var("USER").unwrap_or_default())
        .map(|user| user.id());

    let common_app_paths = vec!["/usr/bin", "/usr/local/bin", "/opt"];

    let system_daemons = vec!["systemd", "xwayland", "isolated web co", "pipewire-pulse", "wireplumber", "dbus-daemon", "sshd","hyprland", "kworker"];

    let mut processes: Vec<String> = Vec::new();

    if let Some(uid) = current_user_uid {
        for (_pid, process) in system.processes() {
            let process_name = process.name().to_lowercase();
            let process_path = process.exe().to_string_lossy();
            
            if process.user_id() == Some(uid) &&
            !system_daemons.contains(&process_name.as_str()) &&
            common_app_paths.iter().any(|path| process_path.starts_with(path)) &&
            process.cpu_usage() > 0.1
            {
                let process_name = process.name().to_lowercase();
            if !processes.contains(&process_name) {
                processes.push(process_name.clone());
}

            }
        }
    } else {
        println!("Could not retrieve the current user's UID.");
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
