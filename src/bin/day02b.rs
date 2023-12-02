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
    counts: Vec<CubeCount>,
}

impl Game {
    fn min_cubes(&self) -> CubeCount {
        self.counts
            .iter()
            .fold(CubeCount::default(), |acc, count| acc.max(count))
    }
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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CubeCount {
    /// The discriminants of Cube are the indexes into the array.
    count: [u32; 3],
}

impl CubeCount {
    pub fn new(red: u32, green: u32, blue: u32) -> Self {
        Self::default()
            .increment(Cube::Red, red)
            .increment(Cube::Green, green)
            .increment(Cube::Blue, blue)
    }

    fn increment(&self, cube: Cube, amount: u32) -> Self {
        let mut cube_count = self.clone();
        cube_count.count[cube as usize] += amount;
        cube_count
    }

    fn max(&self, other: &CubeCount) -> Self {
        let mut count = [0_u32; 3];
        for (i, this_count) in self.count.into_iter().enumerate() {
            count[i] = this_count.max(other.count[i]);
        }
        Self { count }
    }

    fn product(&self) -> u32 {
        self.count.iter().product()
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
            counts: cube_counts,
        })
    }

    // Vec<Result<...>> to Result<Vec<...>>
    lines
        .map(parse_line)
        .collect::<Result<Vec<Game>, Box<dyn Error>>>()
}

pub(crate) fn total(lines: Lines) -> Result<u32, Box<dyn Error>> {
    let games = parse(lines)?;
    let total = games.iter().map(|game| game.min_cubes().product()).sum();
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
    fn example_min_cubes() {
        let games = parse(EXAMPLE.lines()).expect("example");
        let min_cubes: Vec<CubeCount> = games.iter().map(|game| game.min_cubes()).collect();
        assert_eq!(min_cubes[0], CubeCount::new(4, 2, 6));
        assert_eq!(min_cubes[1], CubeCount::new(1, 3, 4));
        assert_eq!(min_cubes[2], CubeCount::new(20, 13, 6));
        assert_eq!(min_cubes[3], CubeCount::new(14, 3, 15));
        assert_eq!(min_cubes[4], CubeCount::new(6, 3, 2));
    }

    #[test]
    fn product() {
        let product = CubeCount::new(4, 2, 6).product();
        assert_eq!(product, 48);
    }

    #[test]
    fn total_example() {
        let lines = EXAMPLE.lines();
        let answer = total(lines).expect("answer");
        assert_eq!(answer, 2286);
    }
}
