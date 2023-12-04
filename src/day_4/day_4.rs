use std::collections::HashSet;
use regex::Regex;
use crate::utils::read_file;

pub fn day_4() {
    let lines = read_file::read_lines("day4");
    let original_set_of_cards: Vec<Card> = lines.iter()
        .map(|line| Card::from(line))
        .collect();

    let total_points: i32 = original_set_of_cards.iter()
        .map(|card| card.get_score())
        .sum();

    println!("Part 1: {total_points}");
}

#[derive(Debug)]
struct Card {
    id: i16,
    winning_numbers: Vec<i8>,
    numbers: Vec<i8>,
}

impl Card {
    fn from(line: &String) -> Card {
        let regex = Regex::new(r"Card[ ]*([0-9]*):").unwrap();
        let id = regex.captures(line).unwrap().get(1).unwrap().as_str().parse::<i16>().unwrap();
        let all_numbers: Vec<&str> = line.split(":").nth(1).unwrap().split("|").collect();

        let winning_numbers: Vec<i8> = all_numbers[0].trim().split_whitespace()
            .into_iter()
            .map(|num| num.parse::<i8>().unwrap())
            .collect();

        let numbers: Vec<i8> = all_numbers[1].trim().split_whitespace()
            .into_iter()
            .map(|num| num.parse::<i8>().unwrap())
            .collect();

        return Card {
            id,
            winning_numbers,
            numbers,
        };
    }

    fn get_score(&self) -> i32 {
        let count_matching_numbers = self.get_count_of_winning_numbers();

        let mut total_score = 0;
        if count_matching_numbers >= 1 {
            total_score = 1;
            for _ in 1..=count_matching_numbers - 1 {
                total_score = total_score * 2
            }
        }

        return total_score;
    }

    fn get_count_of_winning_numbers(&self) -> usize {
        self.winning_numbers.iter().collect::<HashSet<_>>()
            .intersection(&self.numbers.iter().collect::<HashSet<_>>())
            .count()
    }
}
