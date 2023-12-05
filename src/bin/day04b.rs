use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::read_to_string;
use std::str::Lines;

type Game = (Vec<u32>, Vec<u32>);

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_to_string("data/day04.txt").expect("file");
    let lines = contents.lines();
    let answer = process(lines).expect("processed");
    println!("Answer: {answer}");
    Ok(())
}

fn parse_numbers(text: &str) -> Vec<u32> {
    text.split_whitespace()
        .map(|num| num.parse().expect("number"))
        .collect()
}

fn parse(lines: Lines) -> BTreeMap<usize, Game> {
    let pairs = lines
        .map(|line| {
            line.split_once(':')
                .expect("line with colon")
                .1
                .split_once('|')
                .expect("line with pipe")
        })
        .map(|(winning, card)| (parse_numbers(winning), parse_numbers(card)));
    pairs.enumerate().collect()
}

fn process_games(games: BTreeMap<usize, Game>) -> u32 {
    let mut counts = vec![1_u32; games.len()];

    for (id, game) in games {
        let winning: BTreeSet<u32> = game.0.clone().into_iter().collect();
        let card: BTreeSet<u32> = game.1.clone().into_iter().collect();
        let winning_count = winning.intersection(&card).count();

        // Add duplicates
        for dup_index in (id + 1)..=(id + winning_count) {
            if dup_index < counts.len() {
                counts[dup_index] += counts[id];
            }
        }
    }

    counts.iter().sum()
}

fn process(lines: Lines) -> Result<u32, Box<dyn Error>> {
    let games = parse(lines);
    Ok(process_games(games))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn example_answer() {
        let lines = EXAMPLE.lines();
        let answer = process(lines).expect("answer");
        assert_eq!(answer, 30);
    }

    #[test]
    fn process_game_basic() {
        let game = "Game 1: 2 | 3";
        let games = parse(game.lines());
        assert_eq!(process_games(games), 1);
        let game = "Game 1: 2 | 2\nGame 2: 2 | 2";
        let games = parse(game.lines());
        assert_eq!(process_games(games), 3);
    }
}
