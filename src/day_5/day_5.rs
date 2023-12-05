use std::collections::HashMap;

use crate::utils::read_file;

pub fn day_5() {
    let lines = read_file::read_lines("day5");

    let seeds: Vec<&str> = lines[0].split(":").collect();
    let seeds: Vec<&str> = seeds[1].trim().split(" ").collect();
    let seeds: Vec<i64> = seeds.iter().map(|seed| seed.parse::<i64>().unwrap()).collect();

    let mut raw_map: HashMap<&str, Vec<&String>> = HashMap::new();

    let mut current_map_name: &str = "";

    for line in &lines[1..lines.len()] {
        if line.is_empty() {
            current_map_name = "";
            continue;
        }
        if line.chars().nth(0).unwrap().is_alphabetic() {
            let map_name: Vec<&str> = line.split(" map").collect();
            let map_name: &str = map_name[0];
            current_map_name = map_name;
            raw_map.insert(current_map_name, vec!());
        }
        if line.chars().nth(0).unwrap().is_digit(10) {
            let current_value: &Vec<&String> = raw_map.get(current_map_name).unwrap();
            let mut new_value = current_value.clone();
            new_value.push(line);
            raw_map.insert(current_map_name, new_value);
        }
    }

    let map: HashMap<&str, Map> = raw_map.iter()
        .map(|(key, value)|
            (
                *key,
                Map::new(value.iter().map(|mapping| {
                    let values: Vec<&str> = mapping.split_whitespace().collect();
                    let destination_range_start = values[0].parse::<i64>().unwrap();
                    let source_range_start = values[1].parse::<i64>().unwrap();
                    let range_length = values[2].parse::<i64>().unwrap();
                    Mapping::new(destination_range_start, source_range_start, range_length)
                }
                ).collect())
            )
        )
        .collect();

    let min_location =seeds.iter().map(|seed| seed_to_location(*seed, &map))
        .min()
        .unwrap();

    println!("{min_location}");
}

fn seed_to_location(seed: i64, map: &HashMap<&str, Map>) -> i64 {
    let soil = map.get("seed-to-soil").unwrap().get_value(seed);
    let fertilizer = map.get("soil-to-fertilizer").unwrap().get_value(soil);
    let water = map.get("fertilizer-to-water").unwrap().get_value(fertilizer);
    let light = map.get("water-to-light").unwrap().get_value(water);
    let temperature = map.get("light-to-temperature").unwrap().get_value(light);
    let humidity = map.get("temperature-to-humidity").unwrap().get_value(temperature);
    let location = map.get("humidity-to-location").unwrap().get_value(humidity);
    location
}

#[derive(Debug)]
struct Map {
    mappings: Vec<Mapping>,
}

impl Map {
    pub fn new(mappings: Vec<Mapping>) -> Self {
        Self { mappings }
    }

    fn get_value(&self, source: i64) -> i64 {
        let matching_mappings: Vec<&Mapping> = self.mappings.iter()
            .filter(|mapping| (mapping.source_range_start..(mapping.source_range_start + mapping.range_length)).contains(&source))
            .collect();
        matching_mappings.get(0).map(|mapping| mapping.map(source)).unwrap_or(source)
    }
}

#[derive(Debug)]
struct Mapping {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

impl Mapping {
    pub fn new(destination_range_start: i64, source_range_start: i64, range_length: i64) -> Self {
        Self { destination_range_start, source_range_start, range_length }
    }

    fn map(&self, source: i64) -> i64 {
        self.destination_range_start + (source - self.source_range_start)
    }
}