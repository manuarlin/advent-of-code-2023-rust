use std::cmp::{max, min};
use std::collections::HashMap;

use crate::utils::read_file;

pub fn day_5() {
    let lines = read_file::read_lines("day5");

    let seeds_input: Vec<&str> = lines[0].split(":").collect();
    let seeds_input: Vec<&str> = seeds_input[1].trim().split(" ").collect();

    let mut seed_ranges: Vec<(i64, i64)> = vec!();
    for i in (0..seeds_input.len()).step_by(2) {
        let start_seed_range = seeds_input[i].parse::<i64>().unwrap();
        let end_seed_range = start_seed_range + seeds_input[i + 1].parse::<i64>().unwrap();
        seed_ranges.push((start_seed_range, end_seed_range));
    }

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
        .map(|(key, value)| {
            let mut mappings: Vec<Mapping> = value.iter().map(|mapping| {
                let values: Vec<&str> = mapping.split_whitespace().collect();
                let destination_range_start = values[0].parse::<i64>().unwrap();
                let source_range_start = values[1].parse::<i64>().unwrap();
                let range_length = values[2].parse::<i64>().unwrap();
                Mapping::new(destination_range_start, source_range_start, range_length)
            }).collect();
            mappings.sort_by(|mapping1, mapping2| mapping1.source_range_start.partial_cmp(&mapping2.source_range_start).unwrap());
            (*key, Map::new(mappings))
        })
        .collect();

    let see_to_soil_mapper = map.get("seed-to-soil").unwrap();
    let soil_to_fertilizer_mapper = map.get("soil-to-fertilizer").unwrap();
    let fertilizer_to_water_mapper = map.get("fertilizer-to-water").unwrap();
    let water_to_light_mapper = map.get("water-to-light").unwrap();
    let light_to_temperature_mapper = map.get("light-to-temperature").unwrap();
    let temperature_to_humidity_mapper = map.get("temperature-to-humidity").unwrap();
    let humidity_to_location_mapper = map.get("humidity-to-location").unwrap();

    let location_ranges: Vec<(i64, i64)> = seed_ranges.iter()
        .flat_map(|seed_range| {
            let soil_ranges = see_to_soil_mapper.map_range(vec!(*seed_range));
            let fertilizer_ranges = soil_to_fertilizer_mapper.map_range(soil_ranges);
            let water_ranges = fertilizer_to_water_mapper.map_range(fertilizer_ranges);
            let light_ranges = water_to_light_mapper.map_range(water_ranges);
            let temperature_ranges = light_to_temperature_mapper.map_range(light_ranges);
            let humidity_ranges = temperature_to_humidity_mapper.map_range(temperature_ranges);
            let location_ranges = humidity_to_location_mapper.map_range(humidity_ranges);
            location_ranges
        })
        .collect();

    let min_location_ranges = location_ranges.iter()
        .map(|(start, _)| start)
        .min().unwrap();

    println!("{min_location_ranges}")
}

#[derive(Debug)]
struct Map {
    mappings: Vec<Mapping>,
}

impl Map {
    pub fn new(mappings: Vec<Mapping>) -> Self {
        Self { mappings }
    }

    fn map_range(&self, ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
        let mut result: Vec<(i64, i64)> = vec!();
        for range in ranges {
            let mappings = &self.mappings;
            let matching_mappings: Vec<((i64, i64), (i64, i64))> = mappings.iter()
                .map(|mapping| mapping.map_range(range))
                .filter(|result| result.is_some())
                .map(|result| result.unwrap())
                .collect();

            if matching_mappings.len() == 0 {
                result.push(range);
                continue;
            }

            let intersections: Vec<(i64, i64)> = matching_mappings.iter().map(|(intersection, _)| *intersection)
                .collect();
            for (_, result_mapping) in &matching_mappings {
                result.push(*result_mapping)
            }

            let remaining_ranges = sub_intersections(range, intersections);
            for remaining in remaining_ranges {
                result.push(remaining)
            }
        }
        result
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

    fn map_range(&self, range: (i64, i64)) -> Option<((i64, i64), (i64, i64))> {
        let intersection = intersection(
            range,
            (self.source_range_start, self.source_range_start + self.range_length),
        )?;
        let (start_intersection, end_intersection) = intersection;
        Some((intersection, (self.map(start_intersection), self.map(end_intersection))))
    }
}

fn intersection(range: (i64, i64), mapping_range: (i64, i64)) -> Option<(i64, i64)> {
    let (range_start, range_end) = range;
    let (mapping_range_start, mapping_range_end) = mapping_range;

    if mapping_range_start > range_end {
        return None;
    }
    if mapping_range_end < range_start {
        return None;
    }
    Some((max(mapping_range_start, range_start), min(mapping_range_end, range_end)))
}

fn sub_intersections(range: (i64, i64), intersections: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let (range_start, range_end) = range;
    let mut result = vec!();
    let (start_first_intersection, _) = intersections[0];
    let (_, end_last_intersection) = *intersections.last().unwrap();
    if range_start < start_first_intersection {
        result.push((range_start, start_first_intersection - 1))
    }

    for i in (0..intersections.len()).step_by(2) {
        let (_, end_intersection_1) = intersections[i];
        let start_intersection_2: i64 = match intersections.get(i + 1) {
            Some((start_intersection, _)) => *start_intersection,
            None => break
        };
        if start_intersection_2 > end_intersection_1 {
            result.push((end_intersection_1 + 1, start_intersection_2 - 1))
        }
    }

    if range_end > end_last_intersection {
        result.push((end_last_intersection + 1, range_end))
    }
    result
}
