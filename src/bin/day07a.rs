use std::cmp::Ordering;
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

        let max_count = *counts.values().max().unwrap() as u32;
        if max_count == 5 {
            Self::FiveOfAKind
        } else if max_count == 4 {
            Self::FourOfAKind
        } else if max_count == 3 && counts.iter().any(|(_, count)| *count == 2) {
            Self::FullHouse
        } else if max_count == 3 {
            Self::ThreeOfAKind
        } else if max_count == 2 && counts.iter().filter(|(_, count)| **count == 2).count() == 2 {
            Self::TwoPair
        } else if max_count == 2 {
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
                    '2'..='9' => c.to_digit(10).unwrap(),
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
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
    // data.iter().enumerate().for_each(|(rank, hand)| println!("Rank {:?}, hand {hand:?}, winnings {}", hand.rank, hand.bid * (rank + 1) as u32));
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
        assert_eq!(answer, 6440);
    }
}
