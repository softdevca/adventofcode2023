use std::collections::BTreeSet;
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

fn parse(lines: Lines) -> Vec<Game> {
    lines
        .map(|line| {
            line.split_once(':')
                .expect("line with colon")
                .1
                .split_once('|')
                .expect("line with pipe")
        })
        .map(|(winning, card)| (parse_numbers(winning), parse_numbers(card)))
        .collect()
}

fn process_game(game: &Game) -> u32 {
    let winning: BTreeSet<u32> = game.0.clone().into_iter().collect();
    let card: BTreeSet<u32> = game.1.clone().into_iter().collect();
    (1 << (winning.intersection(&card).count() as u32)) >> 1
}

fn process(lines: Lines) -> Result<u32, Box<dyn Error>> {
    let games = parse(lines);
    Ok(games.iter().map(process_game).sum())
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
        assert_eq!(answer, 13);
    }

    #[test]
    fn process_game_basic() {
        let game = "Game 1: 2 | 3";
        let game = parse(game.lines());
        assert_eq!(process_game(game.first().expect("at least one game")), 0);
        let game = "Game 1: 2 | 2";
        let game = parse(game.lines());
        assert_eq!(process_game(game.first().expect("at least one game")), 1);
        let game = "Game 1: 2 3 | 2 3";
        let game = parse(game.lines());
        assert_eq!(process_game(game.first().expect("at least one game")), 2);
    }
}
