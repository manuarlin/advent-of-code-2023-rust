use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
    hand_type: HandType,
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        return self.cards == other.cards;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type > other.hand_type {
            return Some(Ordering::Greater);
        }
        if self.hand_type < other.hand_type {
            return Some(Ordering::Less);
        }
        for (index, card) in self.cards.iter().enumerate() {
            let other_card = &other.cards[index];
            if card > other_card {
                return Some(Ordering::Greater);
            } else if card < other_card {
                return Some(Ordering::Less);
            }
        }
        Some(Ordering::Equal)
    }
}

impl Hand {
    pub fn new(cards: [Card; 5], bid: i64) -> Self {
        let hand_type: HandType = Hand::compute_hand_type_with_joker(&cards);
        Self { cards, bid, hand_type }
    }

    fn compute_hand_type_without_joker(cards: &[Card; 5]) -> HandType {
        let mut map_of_equals: HashMap<&Card, i8> = HashMap::new();

        for card in cards {
            if !map_of_equals.contains_key(card) {
                map_of_equals.insert(card, 1);
            } else {
                let current_value = *map_of_equals.get(card).unwrap();
                map_of_equals.insert(card, current_value + 1);
            }
        }

        let mut number_of_equals: Vec<i8> = map_of_equals.iter()
            .map(|(_, value)| *value)
            .collect();

        number_of_equals.sort();

        let max_number_of_equals = *number_of_equals.last().unwrap();

        if max_number_of_equals == 5 {
            return HandType::FiveOfAKind;
        }
        if max_number_of_equals == 4 {
            return HandType::FourOfAKind;
        }
        if max_number_of_equals == 3 {
            return if *number_of_equals.get(number_of_equals.len() - 2).unwrap() > 1 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            };
        }
        if max_number_of_equals == 2 {
            return if *number_of_equals.get(number_of_equals.len() - 2).unwrap() > 1 {
                HandType::TwoPairs
            } else {
                HandType::OnePair
            };
        }
        HandType::HighCard
    }

    fn compute_hand_type_with_joker(cards: &[Card; 5]) -> HandType {
        if !cards.contains(&J) {
            return Hand::compute_hand_type_without_joker(cards);
        } else {
            let mut best_hand_type = Hand::compute_hand_type_without_joker(&cards);
            for joker_replacement_try in Card::iter() {
                let mut new_cards: [Card; 5] = [J, J, J, J, J];
                for (i, card) in cards.iter().enumerate() {
                    if card == &J {
                        new_cards[i] = joker_replacement_try;
                    } else {
                        new_cards[i] = *card;
                    }
                }
                let new_cards_hand_type = Hand::compute_hand_type_without_joker(&new_cards);

                if new_cards_hand_type > best_hand_type {
                    best_hand_type = new_cards_hand_type;
                }
            }
            best_hand_type
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPairs = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, EnumIter, Clone, Copy)]
enum Card {
    J = 1,
    Number2 = 2,
    Number3 = 3,
    Number4 = 4,
    Number5 = 5,
    Number6 = 6,
    Number7 = 7,
    Number8 = 8,
    Number9 = 9,
    T = 10,
    Q = 11,
    K = 12,
    A = 13,
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


