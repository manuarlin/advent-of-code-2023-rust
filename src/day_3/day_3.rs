use std::cmp::min;

use crate::utils::read_file;

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
            number_value = String::from("");
        }
    }

    let mut part_numbers_values: Vec<i64> = vec!();

    for number in numbers {
        let (start_coordinates_i, start_coordinates_j) = number.start_coordinates;
        let (end_coordinates_i, _) = number.end_coordinates;
        for line in &lines[start_coordinates_j.saturating_sub(1)..=min(lines_length - 1, start_coordinates_j + 1)] {
            for char in &line.as_bytes()[start_coordinates_i.saturating_sub(1)..=min(end_coordinates_i + 1, line_length - 1)] {
                let char = *char as char;
                if char != '.' && !char.is_digit(10) {
                    part_numbers_values.push(number.value)
                }
            }
        }
    }

    let sum: i64 = part_numbers_values.iter().sum();
    println!("{sum}")
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