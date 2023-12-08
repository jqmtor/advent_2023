use std::{collections::HashMap, cmp::Ordering};

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Number(u32),
}

use Card::*;

impl Card {
    fn strength(&self) -> u32 {
        match self {
            Number(n) => *n,
            Jack => 11,
            Queen => 12,
            King => 13,
            Ace => 14,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

impl HandType {
    // the concrete strength values do not matter, they just need
    // to model the correct relative strength.
    fn strength(&self) -> u32 {
        match self {
            FiveOfAKind => 7,
            FourOfAKind => 6,
            FullHouse => 5,
            ThreeOfAKind => 4,
            TwoPair => 3,
            OnePair => 2,
            HighCard => 1,
        }
    }
}

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub hand_type: HandType,
}

use HandType::*;

impl From<&str> for Hand {
    fn from(hand_str: &str) -> Self {
        let cards = hand_str.chars().map({|c| 
            match c {
                'A' => Ace,
                'K' => King,
                'Q' => Queen,
                'J' => Jack,
                'T' => Number(10),
                _ => Number(c.to_digit(10).expect("Number to be parseable")),
            }
        });
        let occurrences = cards.clone().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        let contains_three_of_a_kind = occurrences
            .iter()
            .any(|(_, occur)| { *occur == 3 });
        let contains_pair = occurrences.iter().any(|(_, occur)| { *occur == 2 });

        let hand_type = if occurrences.iter().any(|(_, occur)| { *occur == 5 }) {
            FiveOfAKind
        } else if occurrences.iter().any(|(_, occur)| { *occur == 4 }) {
            FourOfAKind
        } else if contains_three_of_a_kind && contains_pair {
            FullHouse
        } else if contains_three_of_a_kind {
            ThreeOfAKind
        } else if occurrences.iter().filter(|(_, occur)| { **occur == 2 }).count() == 2 {
            TwoPair
        } else if contains_pair {
            OnePair
        } else {
            HighCard
        };
        Hand { cards: cards.collect(), hand_type }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.hand_type == other.hand_type &&
            self.cards.iter().zip(other.cards.iter()).all(|(s, o)| { s == o })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        if self.hand_type.strength() > other.hand_type.strength() {
            Some(Ordering::Greater)
        } else {
            for (card, other) in self.cards.iter().zip(other.cards.iter()) { 
                match card.strength().cmp(&other.strength()) {
                    Ordering::Equal => { continue; },
                    ord => { return Some(ord); },
                }
            }
            Some(Ordering::Equal)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_pair_hand_from_string() {
        let str = "32T3K";
        let expected = Hand {
            cards: vec![Number(3), Number(2), Number(10), Number(3), King],
            hand_type: OnePair,
        };
        assert_eq!(Hand::from(str), expected);
    }

    #[test]
    fn test_three_of_a_kind_hand_from_string() {
        let str = "QQQJA";
        let expected = Hand {
            cards: vec![Queen, Queen, Queen, Jack, Ace],
            hand_type: ThreeOfAKind,
        };
        assert_eq!(Hand::from(str), expected);
    }

    #[test]
    fn test_hand_strength() {
        let first_hand = Hand {
            cards: vec![Queen, Queen, Queen, Jack, Ace],
            hand_type: ThreeOfAKind,
        };

        let second_hand = Hand {
            cards: vec![Queen, Queen, Queen, Jack, King],
            hand_type: ThreeOfAKind,
        };
        assert!(first_hand > second_hand);
        
    }
}
