use crate::db_reader::{treat_app, treat_day};
use std::{cell::RefCell, rc::Rc};

#[allow(dead_code)]
pub fn data_visualizer() -> String {
    format!("")
}

pub fn formatg_v(values: Vec<(String, u8)>) -> String {
    let mut graph = String::from("");
    for j in (1..12).rev() {
         values.iter().map(|(_,i)| {
            graph.push(' ');
            if j > 1 {
                match i + 20 > j * 10 {
                    true => {
                        graph.push_str("███");
                    }
                    false => {
                        graph.push_str("   ");
                    }
                }
            } else {
                graph.push_str(format!("{:<3}", i).as_str());
            }
        });
        graph.push('\n');
    }
    values.iter().map(|(m,_)| {
        graph.push_str(format!(" {:2} ", m).as_str());
    });

    graph.push_str(format!("{:?}", values).as_str());

    graph
}

pub fn formatg_h(values: Vec<(String, u8)>) -> String {
    let mut graph = String::new();
    let longest = values.iter().map(|(i, _k)| i.len()).max().unwrap_or(0);

    for (i, k) in &values {
        graph.push_str(&format!("\n{:<width$} ", i, width = longest));
        for _ in 0..(k / 5) + 3 {
            graph.push('▇');
        }
    }

    graph
}

pub fn app_usage(app: &String, day: &String) -> String {
    let mut treated_value = treat_app(day, app);
    String::from("")
}

pub fn app_graph(app: &String, day: &String, hrztl: bool) -> String {
    let treated_value = treat_app(day, app);
    if hrztl {
        formatg_h(treated_value)
    } else {
        formatg_v(treated_value)
    }
}

pub fn power_usage(day: &String) -> String {
    let mut treated_value = treat_day(day);
    String::from("")
}

pub fn power_graph(day: &String, hrztl: bool) -> String {
    let mut treated_value = treat_day(day);
    if hrztl {
        //formatg_h(treated_value)
        format!("")
    } else {
        formatg_v(treated_value)
    }
}

pub fn increase_day(day: Rc<RefCell<String>>) {}

pub fn decrease_day(day: Rc<RefCell<String>>) {}
