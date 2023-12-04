use std::collections::{HashMap, HashSet};

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

    let mut map_of_cards: HashMap<usize, &Card> = HashMap::new();

    for card in &original_set_of_cards {
        map_of_cards.insert(card.id, card);
    }

    let mut map_of_copies: HashMap<usize, Vec<&Card>> = HashMap::new();

    for card in &original_set_of_cards {
        let count_of_winning_numbers = card.get_count_of_winning_numbers();

        let mut copies: Vec<&Card> = vec!();
        for i in 1..=count_of_winning_numbers {
            let copy_id = card.id + i;
            copies.push(map_of_cards.get(&copy_id).unwrap())
        }
        map_of_copies.insert(card.id, copies);
    }

    let mut all_cards: Vec<&Card> = vec!();

    for card in &original_set_of_cards {
        all_cards.push(card);
    }

    for card in &original_set_of_cards {
        for copy in card.get_all_copies(&map_of_copies) {
            all_cards.push(copy)
        }
    }

    let total = all_cards.iter().count();

    println!("Part 2: {total}")

}

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: Vec<i8>,
    numbers: Vec<i8>,
}

impl Card {
    fn from(line: &String) -> Card {
        let regex = Regex::new(r"Card[ ]*([0-9]*):").unwrap();
        let id = regex.captures(line).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();
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

    fn get_all_copies<'a>(&'a self, map_of_copies: &HashMap<usize, Vec<&'a Card>>) -> Vec<&Card> {
        let copies = map_of_copies.get(&self.id).unwrap();
        let mut all_copies: Vec<&Card> = vec!();

        for copy in copies {
            all_copies.push(copy);
            for sub_copy in copy.get_all_copies(map_of_copies) {
                all_copies.push(sub_copy)
            }
        }
        all_copies
    }
}
