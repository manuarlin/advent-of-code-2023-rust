use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;

use crate::day_7::day_7::Card::{A, J, K, Number2, Number3, Number4, Number5, Number6, Number7, Number8, Number9, Q, T};
use crate::utils::read_file;

pub fn day_7() {
    let lines = read_file::read_lines("day7");
    let mut hands: Vec<Hand> = lines.iter()
        .map(|hand_line| {
            let hand_line_parts: Vec<&str> = hand_line.split_whitespace().collect();
            let cards_part = hand_line_parts[0];
            let bid_part = hand_line_parts[1];

            let mut cards: [Card; 5] = [Number2, Number2, Number2, Number2, Number2];

            for (index, card_char) in cards_part.as_bytes().iter().enumerate() {
                let card_char = *card_char as char;
                cards[index] = Card::from(card_char);
            }
            let bid = bid_part.parse::<i64>().unwrap();
            Hand::new(cards, bid)
        })
        .collect();

    hands.sort_by(|hand1, hand2| hand1.partial_cmp(hand2).unwrap().reverse());

    let mut total: i64 = 0;

    for (index, hand) in hands.iter().enumerate() {
        total = total + (hands.len() as i64 - index as i64) * hand.bid;
    }

    println!("{total}")
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: i64,
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.cards == other.cards;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type() > other.hand_type() {
            return Some(Ordering::Greater);
        }
        if self.hand_type() < other.hand_type() {
            return Some(Ordering::Less);
        }

        for (index, card) in self.cards.iter().enumerate() {
            if card > &other.cards[index] {
                return Some(Ordering::Greater);
            } else if card < &other.cards[index] {
                return Some(Ordering::Less);
            }
        }
        Some(Ordering::Equal)
    }
}

impl Hand {
    pub fn new(cards: [Card; 5], bid: i64) -> Self {
        Self { cards, bid }
    }

    fn hand_type(&self) -> HandType {
        let mut map_of_equals: HashMap<&Card, i8> = HashMap::new();

        for card in &self.cards {
            if !map_of_equals.contains_key(card) {
                map_of_equals.insert(card, 1);
            } else {
                let current_value = map_of_equals.get(card).unwrap();
                map_of_equals.insert(card, *current_value + 1);
            }
        }

        let mut number_of_equals: Vec<&i8> = map_of_equals.iter()
            .map(|(_, value)| value)
            .collect();

        number_of_equals.sort();

        let max_number_of_equals = *number_of_equals.last().unwrap();

        if max_number_of_equals == &5 {
            return HandType::FiveOfAKind;
        }
        if max_number_of_equals == &4 {
            return HandType::FourOfAKind;
        }
        if max_number_of_equals == &3 {
            return if *number_of_equals.get(number_of_equals.len() - 2).unwrap() > &1 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            };
        }
        if max_number_of_equals == &2 {
            return if *number_of_equals.get(number_of_equals.len() - 2).unwrap() > &1 {
                HandType::TwoPairs
            } else {
                HandType::OnePair
            };
        }
        HandType::HighCard
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Hash)]
enum Card {
    Number2,
    Number3,
    Number4,
    Number5,
    Number6,
    Number7,
    Number8,
    Number9,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from(char: char) -> Card {
        match char {
            'A' => A,
            'K' => K,
            'Q' => Q,
            'J' => J,
            'T' => T,
            '9' => Number9,
            '8' => Number8,
            '7' => Number7,
            '6' => Number6,
            '5' => Number5,
            '4' => Number4,
            '3' => Number3,
            '2' => Number2,
            _ => Number2
        }
    }
}
