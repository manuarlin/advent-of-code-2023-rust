use std::fs;

pub fn read_lines(file_name: &str) -> Vec<String> {
    let file_path = format!("inputs/{}", file_name);
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
