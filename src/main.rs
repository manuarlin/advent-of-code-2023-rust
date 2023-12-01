use crate::utils::read_file;

pub mod utils;

fn main() {
    let lines = read_file::read_lines("day1");

    let result = day_1(lines);

    println!("{}", result)
}

fn day_1(lines: Vec<String>) -> u32 {
    let number_per_line: Vec<u32> = lines.iter()
        .map(|line| line.chars().collect())
        .map(|chars: Vec<char>| extract_digits(chars))
        .map(|digits: Vec<char>| vec!(*digits.get(0).unwrap(), *digits.get(digits.len() - 1).unwrap()))
        .map(|two_digit_number_chars: Vec<char>| {
            let mut two_digit_number = String::from(*two_digit_number_chars.get(0).unwrap());
            two_digit_number.push(*two_digit_number_chars.get(1).unwrap());
            two_digit_number.parse::<u32>().unwrap()
        })
        .collect();

    number_per_line.iter().sum()
}

// TODO Try to improve it
fn extract_digits(chars: Vec<char>) -> Vec<char> {
    let mut  digit_in_words_replacement: String = chars.into_iter().collect();
    // Need to reinsert digits as word when letter is shared among several possibilities like zoneight234
    digit_in_words_replacement = digit_in_words_replacement.replace("one", "one1one");
    digit_in_words_replacement = digit_in_words_replacement.replace("two", "two2two");
    digit_in_words_replacement = digit_in_words_replacement.replace("three", "three3three");
    digit_in_words_replacement = digit_in_words_replacement.replace("four", "four4four");
    digit_in_words_replacement = digit_in_words_replacement.replace("five", "five5five");
    digit_in_words_replacement = digit_in_words_replacement.replace("six", "six6six");
    digit_in_words_replacement = digit_in_words_replacement.replace("seven", "seven7seven");
    digit_in_words_replacement = digit_in_words_replacement.replace("eight", "eight8eight");
    digit_in_words_replacement = digit_in_words_replacement.replace("nine", "nine9nine");
    digit_in_words_replacement
        .chars()
        .filter(|char: &char| char.is_digit(10))
        .collect()
}



