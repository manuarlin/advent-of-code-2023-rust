use regex::Regex;

use crate::utils::read_file;

pub mod utils;

fn main() {
    let lines = read_file::read_lines("day2");

    part_2(lines);
}

fn part_2(lines: Vec<String>) {
    let games: Vec<Game> = lines.iter()
        .map(|line| Game::from(line))
        .collect();

    let possible_games = games.iter()
        .filter(|game| game.is_possible());
    let part_1_result: i16 = possible_games.fold(0, |acc, e| acc + i16::from(e.id));
    println!("{part_1_result}");

    let minimum_sets_powers = games.iter()
        .map(|game| game.minimum_set_power());
    let part_2_result: i32 = minimum_sets_powers.fold(0, |acc, e| acc + e);
    println!("{part_2_result}");
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

    fn minimum_set_power(&self) -> i32 {
        let minimum_blue_cubes_needed = self.sets.iter()
            .map(|set| set.blue_cubes)
            .max()
            .unwrap_or(0);

        let minimum_red_cubes_needed = self.sets.iter()
            .map(|set| set.red_cubes)
            .max()
            .unwrap_or(0);

        let minimum_green_cubes_needed = self.sets.iter()
            .map(|set| set.green_cubes)
            .max()
            .unwrap_or(0);

        return i32::from(minimum_blue_cubes_needed) * i32::from(minimum_red_cubes_needed) * i32::from(minimum_green_cubes_needed)
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



