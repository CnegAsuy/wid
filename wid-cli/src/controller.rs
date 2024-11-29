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

pub fn value_to_graph_horizontally(values: Vec<u8>) -> String {
    let mut graph = String::from("");
    for i in values.clone() {
        for j in 1..(i/5)+3 {
            if j == 1 {graph.push_str(format!("\n{:<3} ", i).as_str());} 
            else {graph.push('▇')};
        }
    }
    
    graph
}