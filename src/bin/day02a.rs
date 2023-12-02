use std::error::Error;
use std::fs::read_to_string;
use std::io::Error as IoError;
use std::str::Lines;

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_to_string("data/day02.txt").expect("file");
    let lines = contents.lines();
    let total = total(lines)?;
    println!("Total: {total}");
    Ok(())
}

#[derive(Debug)]
struct Game {
    id: u32,
    counts: Vec<CubeCount>,
}

#[repr(u8)]
enum Cube {
    Red = 0,
    Green = 1,
    Blue = 2,
}

impl Cube {
    fn from_name(name: &str) -> Option<Cube> {
        match name {
            "red" => Some(Cube::Red),
            "green" => Some(Cube::Green),
            "blue" => Some(Cube::Blue),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Default)]
struct CubeCount {
    /// The discriminants of Cube are the indexes into the array.
    count: [u32; 3],
}

impl CubeCount {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self::default()
            .increment(Cube::Red, red)
            .increment(Cube::Green, green)
            .increment(Cube::Blue, blue)
    }

    /// All counts must be less than or equal to the counts in the other.
    fn none_greater_than(&self, other: &Self) -> bool {
        self.count
            .iter()
            .zip(other.count.iter())
            .all(|(a, b)| a <= b)
    }

    fn increment(&self, cube: Cube, amount: u32) -> Self {
        let mut cube_count = self.clone();
        cube_count.count[cube as usize] += amount;
        cube_count
    }
}

fn parse(lines: Lines) -> Result<Vec<Game>, Box<dyn Error>> {
    fn parse_line(line: &str) -> Result<Game, Box<dyn Error>> {
        let mut parts = line.trim().split_terminator(&[':', ';']);

        // Game
        let game_part = parts.next().ok_or(IoError::other("empty line"))?;
        if !game_part.starts_with("Game ") {
            return Err(IoError::other("line did not start with 'Game'").into());
        }
        let game_id: u32 = game_part["Game ".len()..].parse()?;

        // Counts
        let mut cube_counts = Vec::new();
        for count_spec in parts {
            let mut cube_count = CubeCount::default();
            for cube_spec in count_spec.split(',') {
                if let Some((count_str, cube_name)) = cube_spec.trim().split_once(' ') {
                    let count: u32 = count_str.parse()?;
                    let cube = Cube::from_name(cube_name)
                        .ok_or(IoError::other(format!("unknown cube '{cube_name}'")))?;
                    cube_count = cube_count.increment(cube, count);
                } else {
                    return Err(IoError::other(format!("malformed cube '{cube_spec}'")).into());
                }
            }
            cube_counts.push(cube_count);
        }

        Ok(Game {
            id: game_id,
            counts: cube_counts,
        })
    }

    // Vec<Result<...>> to Result<Vec<...>>
    lines
        .map(parse_line)
        .collect::<Result<Vec<Game>, Box<dyn Error>>>()
}

pub(crate) fn total(lines: Lines) -> Result<u32, Box<dyn Error>> {
    let bag = CubeCount::new(12, 13, 14);
    let games = parse(lines)?;
    let games = games
        .iter()
        .filter(|game| game.counts.iter().all(|count| count.none_greater_than(&bag)));
    let total = games.map(|game| game.id).sum();
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn cube_count() {
        let bag = CubeCount::new(12, 13, 14);
        assert!(!bag.all_greater_than(&bag));
    }

    #[test]
    fn parse_example() {
        let games = parse(EXAMPLE.lines()).expect("answer");
        assert_eq!(games.len(), 5);
        let game1 = games.first().unwrap();
        assert_eq!(game1.id, 1);
        assert_eq!(game1.counts.len(), 3);
        assert_eq!(game1.counts[0].count, [4, 0, 3]);
        assert_eq!(game1.counts[1].count, [1, 2, 6]);
        assert_eq!(game1.counts[2].count, [0, 2, 0]);
        let game5 = games.get(4).unwrap();
        assert_eq!(game5.id, 5);
        assert_eq!(game5.counts.len(), 2);
        assert_eq!(game5.counts[0].count, [6, 3, 1]);
        assert_eq!(game5.counts[1].count, [1, 2, 2]);
    }

    #[test]
    fn read_part_a() {
        let contents = read_to_string("data/day02.txt").expect("file");
        let lines = contents.lines();
        assert_eq!(lines.count(), 100);
    }

    #[test]
    fn total_example() {
        let lines = EXAMPLE.lines();
        let answer = total(lines).expect("answer");
        assert_eq!(answer, 8);
    }
}
