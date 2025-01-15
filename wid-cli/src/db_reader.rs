use serde_json::{json, Value};
use std::{cell::RefCell, fs::File, io::Read, rc::Rc};



pub fn from_date(date: &String, loc: &str) -> Value {
    let mut file = File::open(loc).expect("Cannot find the location of db.json.\nAre you sure the db.json is in the true location or wid-daemon running?");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Unable to read file db.json");
    
    let json_data: Value = serde_json::from_str(&data).unwrap_or_else(|_| json!({}));
    if let Some(value) = json_data.get(date) {
        value.clone()
    } else {
        println!("Warning: Date {date} not found in db.json");
        json!({})
    }
}



// Analyze Data Section Starts

fn analyze_to_v(val: Value, app: &String) -> Vec<(String, u8)> {
    let apps: Rc<RefCell<Vec<(String, u8)>>> = Rc::new(RefCell::new(Vec::new()));

    if let Some(array) = val.as_array() {
        if array.is_empty() {
            println!("Warning: No data available in the array.");
            return vec![];
        }

        prep_array(array, apps.clone());
        fill_array(array, apps.clone(), app.clone());
    } else {
        println!("Warning: Value is not an array.");
    }

    apps.clone()
        .borrow_mut()
        .iter()
        .map(|(name, time)| (name.clone(), ((*time as u16 - 0) * (100 - 0) / (60 - 0)) as u8))
        .collect()
}

fn prep_array(arr: &Vec<Value>, apps: Rc<RefCell<Vec<(String, u8)>>>) {
    let first_hour = arr
        .get(0)
        .and_then(|v| v.get("time").and_then(|x| x.as_str()))
        .and_then(|t| t[..2].parse::<u8>().ok());

    let last_hour = arr
        .last()
        .and_then(|v| v.get("time").and_then(|x| x.as_str()))
        .and_then(|t| t[..2].parse::<u8>().ok());

    if let (Some(start), Some(end)) = (first_hour, last_hour) {
        for i in start..=end {
            apps.borrow_mut().push((i.to_string(), 0));
        }
    } else {
        println!("Warning: Failed to parse time range.");
    }
}


fn fill_array(arr: &[Value], apps: Rc<RefCell<Vec<(String, u8)>>>, selected_app: String) {
    for val in arr {
        let app_array = match val.get("apps").and_then(|x| x.as_array()) {
            Some(array) => array,
            None => continue,
        };

        if !app_array.iter().any(|app| app.as_str() == Some(selected_app.as_str())) {
            continue;
        }

        let time = match val.get("time").and_then(|x| x.as_str()) {
            Some(t) => t,
            None => continue,
        };

        increment_count(&apps, time);
    }
}

fn increment_count(apps: &Rc<RefCell<Vec<(String, u8)>>>, time: &str) {
    let hour = &time[..2];
    for (app_time, count) in apps.borrow_mut().iter_mut() {
        if app_time == hour {
            *count += 1;
        }
    }
}



// Analyze Data Section Ends

// Treating Data 

pub fn treat_app(day: &String, app: &String) -> Vec<(String, u8)> {
    let data = from_date(day, "db.json");
    let x = analyze_to_v(data, app);
    x

}

pub fn treat_day(day: &String) -> Vec<(String, u8)> {
    Vec::new()
}