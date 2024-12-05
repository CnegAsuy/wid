#[allow(dead_code)]
pub fn data_visualizer() -> String {
    format!("")
}

pub fn value_to_graph_vertically(values: Vec<u8>) -> String {
    let mut graph = String::from("");
    for j in (1..12).rev() {
        for i in values.clone() {
            graph.push(' ');
            if j > 1 {
                match i+20 > j * 10 {
                    true => {
                        graph.push_str("███");
                    }
                    false => {
                        graph.push_str("   ");
                    }
                }
            }
            else {
                graph.push_str(format!("{:<3}", i).as_str());
            }
        }
        graph.push('\n');
    }

    graph
}

pub fn value_to_graph_horizontally(values: Vec<(String, u8)>) -> String {
    let mut graph = String::new();
    // Find the longest name length
    let longest = values
        .iter()
        .map(|(i, _k)| i.len())
        .max()
        .unwrap_or(0); // Fallback to 0 if the list is empty

    // Build the graph
    for (i, k) in &values {
        graph.push_str(&format!("\n{:<width$} ", i, width = longest)); // Add the name
        for _ in 0..(k / 5) + 3 {
            graph.push('▇'); // Add blocks
        }
    }

    graph
}