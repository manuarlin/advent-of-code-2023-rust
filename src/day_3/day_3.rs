use std::cmp::min;
use std::collections::HashMap;

use crate::utils::read_file;

#[warn(unused_assignments)]
pub fn day_3() {
    let lines = read_file::read_lines("day3");

    let line_length = lines[0].len();
    let lines_length = lines.len();

    let mut numbers: Vec<Number> = vec!();

    for (j, line) in lines.iter().enumerate() {
        let mut number_value = String::from("");
        let mut number_start_coordinated: (usize, usize) = (0, 0);
        let mut number_end_coordinates: (usize, usize);
        for (i, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                if number_value == "" {
                    number_start_coordinated = (i, j)
                }
                number_value.push(char);
            } else {
                if number_value != "" {
                    number_end_coordinates = (i - 1, j);
                    numbers.push(Number::from(number_value.parse::<i64>().unwrap(), number_start_coordinated, number_end_coordinates));
                    number_value = String::from("");
                }
            }
        }
        if number_value != "" {
            number_end_coordinates = (line_length -1, j);
            numbers.push(Number::from(number_value.parse::<i64>().unwrap(), number_start_coordinated, number_end_coordinates));
        }
    }

    let mut part_numbers_values: Vec<i64> = vec!();

    let mut gears_map: HashMap<(usize, usize), (i64, Option<i64>)> = HashMap::new();

    for number in numbers {
        let (start_coordinates_i, start_coordinates_j) = number.start_coordinates;
        let (end_coordinates_i, _) = number.end_coordinates;
        for i in start_coordinates_j.saturating_sub(1)..=min(lines_length - 1, start_coordinates_j + 1) {
            let line = &lines[i].as_bytes();
            for j in start_coordinates_i.saturating_sub(1)..=min(end_coordinates_i + 1, line_length - 1) {
                let char = line[j] as char;
                if char != '.' && !char.is_digit(10) {
                    part_numbers_values.push(number.value);
                }
                if char == '*' {
                    if gears_map.contains_key(&(i, j)) {
                        let (first_value, _) = gears_map.get(&(i, j)).unwrap();
                        gears_map.insert((i, j), (*first_value, Some(number.value)));
                    } else {
                        gears_map.insert((i, j), (number.value, None));
                    }
                }
            }
        }
    }

    let sum: i64 = part_numbers_values.iter().sum();
    println!("Part 1: {sum}");

    let sum_gear_power: i64 = gears_map.into_iter()
        .filter(|(_, (_, second_value))| second_value.is_some())
        .fold(0, |acc, (_, (first_value, second_value))| acc + first_value * second_value.unwrap());
    println!("Part 2: {sum_gear_power}");
}

struct Number {
    value: i64,
    start_coordinates: (usize, usize),
    end_coordinates: (usize, usize),
}

impl Number {
    fn from(value: i64, start_coordinates: (usize, usize), end_coordinates: (usize, usize)) -> Number {
        Number {
            value,
            start_coordinates,
            end_coordinates,
        }
    }
}