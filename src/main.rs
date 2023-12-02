use regex::Regex;

use crate::utils::read_file;

pub mod utils;

fn main() {
    let lines = read_file::read_lines("day2");

    part_2(lines);
}

fn part_2(lines: Vec<String>) {
    let possible_games: Vec<Game> = lines.iter()
        .map(|line| Game::from(line))
        .filter(|game| game.is_possible())
        .collect();

    let sum_of_ids: i16 = possible_games.iter().fold(0, |acc, e| acc + i16::from(e.id));

    println!("{sum_of_ids}")
}

#[derive(Debug)]
struct Game {
    id: i8,
    sets: Vec<Set>,
}

impl Game {
    fn from(line: &String) -> Game {
        let regex = Regex::new(r"Game ([0-9]+):").unwrap();
        let id = regex.captures(line).unwrap().get(1).unwrap().as_str().parse::<i8>().unwrap();
        let sets = line[8..]
            .split(";")
            .map(|set_part| set_part.trim())
            .map(|set_part| Set::from(set_part))
            .collect();

        return Game {
            id,
            sets,
        };
    }

    fn is_possible(&self) -> bool {
        let possible_sets_count = self.sets.iter().filter(|set| set.is_possible()).count();
        let all_sets_count = self.sets.iter().count();
        return possible_sets_count == all_sets_count;
    }
}

#[derive(Debug)]
struct Set {
    red_cubes: i8,
    green_cubes: i8,
    blue_cubes: i8,
}

impl Set {
    fn from(line: &str) -> Set {
        let mut regex = Regex::new(r"([0-9]+) blue").unwrap();

        let mut blue_cubes: i8 = 0;
        let mut green_cubes: i8 = 0;
        let mut red_cubes: i8 = 0;

        if let Some(captures) = regex.captures(line) {
            blue_cubes = captures.get(1).unwrap().as_str().parse::<i8>().unwrap();
        }

        regex = Regex::new(r"([0-9]+) green").unwrap();
        if let Some(captures) = regex.captures(line) {
            green_cubes = captures.get(1).unwrap().as_str().parse::<i8>().unwrap();
        }

        regex = Regex::new(r"([0-9]+) red").unwrap();
        if let Some(captures) = regex.captures(line) {
            red_cubes = captures.get(1).unwrap().as_str().parse::<i8>().unwrap();
        }

        return Set {
            red_cubes,
            green_cubes,
            blue_cubes,
        };
    }

    fn is_possible(&self) -> bool {
        self.blue_cubes <= 14 && self.green_cubes <= 13 && self.red_cubes <= 12
    }
}



