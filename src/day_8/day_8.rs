use std::collections::HashMap;

use crate::utils::read_file;

pub fn day_8() {
    let lines = read_file::read_lines("day8");

    let directions: Vec<char> = lines[0].as_bytes().iter().map(|char| *char as char).collect();

    let mut nodes: HashMap<String, [String; 2]> = HashMap::new();

    for line in &lines[2..lines.len()] {
        let parts: Vec<&str> = line.split(" = ").collect();
        let node = String::from(parts[0]);
        let directions_part: Vec<&str> = parts[1].split(", ").collect();
        let left = directions_part[0].replace("(", "");
        let left = left.trim();
        let right = directions_part[1].replace(")", "");
        let right = right.trim();
        nodes.insert(node, [String::from(left), String::from(right)]);
    }

    let mut number_of_steps = 0;
    let mut current_nodes: Vec<&str> = nodes.iter().map(|(key, _)| key.as_str())
        .filter(|node| node.ends_with("AAA"))
        .collect();


    while !(current_nodes.iter().all(|node| node.ends_with("Z"))) {
        let nex_direction = directions[number_of_steps % directions.len()];

        let mut next_nodes: Vec<&str> = vec!();

        for node in &current_nodes {
            let next_possible_nodes = nodes.get(*node).unwrap();
            let next_node = if nex_direction == 'L' {
                next_possible_nodes[0].as_str()
            } else {
                next_possible_nodes[1].as_str()
            };
            next_nodes.push(next_node);
        }

        current_nodes = next_nodes;
        number_of_steps = number_of_steps + 1;
    }

    println!("{number_of_steps}");
}