use std::cmp::{max, Ordering};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::read_to_string;
use std::str::Lines;

/// The higher the number the better the card.
type CardRank = u32;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandRank {
    fn from(cards: &[CardRank]) -> Self {
        let mut counts: BTreeMap<CardRank, usize> = BTreeMap::new();
        cards
            .iter()
            .for_each(|card| *counts.entry(*card).or_default() += 1);
        let joker_count = *counts.get(&1).unwrap_or(&0);

        // Counts of non-jokers from highest quantity to least
        let mut plain_counts = counts
            .iter()
            .filter_map(|(card, value)| (*card != 1).then_some(*value))
            .collect::<Vec<usize>>();
        plain_counts.sort();
        plain_counts.reverse();
        let plain_max_count = plain_counts.first().unwrap_or(&0);

        let is_full_house = || {
            // Three cards
            let highest_count = plain_counts.first().unwrap();
            if highest_count + joker_count < 3 {
                return false;
            }

            // Two cards
            let remaining_jokers = joker_count - max(0, 3 - highest_count);
            let next_count = plain_counts.get(1).unwrap();
            next_count + remaining_jokers >= 2
        };

        let is_two_pair = || {
            // First pair
            let highest_count = plain_counts.first().unwrap();
            if highest_count + joker_count < 2 {
                return false;
            }

            // Second pair
            let remaining_jokers = joker_count - max(0, 2 - highest_count);
            let next_count = plain_counts.get(1).unwrap();
            next_count + remaining_jokers >= 2
        };
        if plain_max_count + joker_count > 4 {
            Self::FiveOfAKind
        } else if plain_max_count + joker_count > 3 {
            Self::FourOfAKind
        } else if is_full_house() {
            Self::FullHouse
        } else if plain_max_count + joker_count > 2 {
            Self::ThreeOfAKind
        } else if is_two_pair() {
            Self::TwoPair
        } else if plain_max_count + joker_count > 1 {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<CardRank>,
    bid: u32,
    rank: HandRank,
}

impl Hand {
    fn new(cards: Vec<CardRank>, bid: u32) -> Self {
        let rank = HandRank::from(&cards);
        Self { cards, bid, rank }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.rank == other.rank {
            self.cards
                .iter()
                .zip(&other.cards)
                .map(|(this, other)| this.cmp(other))
                .find(|order| *order != Ordering::Equal)
                .unwrap_or(Ordering::Equal)
        } else {
            (self.rank as isize).cmp(&(other.rank as isize))
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_to_string("data/day07.txt").expect("file");
    let lines = contents.lines();
    let answer = answer(lines)?;
    println!("Answer: {answer}");
    Ok(())
}

fn parse(lines: Lines) -> Vec<Hand> {
    lines
        .map(|line| {
            let split = line.trim().split_once(' ').expect("hand");
            let bid = split.1.parse::<u32>().expect("bid");
            let cards = split
                .0
                .chars()
                .map(|c| match c {
                    'J' => 1,
                    '2'..='9' => c.to_digit(10).unwrap(),
                    'T' => 10,
                    'Q' => 11,
                    'K' => 12,
                    'A' => 13,
                    _ => panic!("Unknown card character '{c}'"),
                })
                .collect();
            Hand::new(cards, bid)
        })
        .collect()
}

fn answer(lines: Lines) -> Result<u32, Box<dyn Error>> {
    let mut data = parse(lines);
    data.sort();
    let winnings = data
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank + 1) as u32)
        .sum();
    Ok(winnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

    #[test]
    fn example_answer() {
        let lines = EXAMPLE.lines();
        let answer = answer(lines).unwrap();
        assert_eq!(answer, 5905);
    }
}
