use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs::read_to_string;
use std::str::Lines;

#[derive(PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Debug for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

#[derive(Debug, PartialEq)]
struct Part {
    coord: Coord,
    number: u32,
}

struct Schematic {
    parts: Vec<Part>,
    symbols: Vec<Coord>,
}

impl Schematic {
    fn any_symbol_touches(&self, part: &Part) -> bool {
        // Symbol must be in this range to touch.
        let x_min = part.coord.x;
        let x_max = x_min + part.number.to_string().len();
        let x_range = (x_min as i32 - 1)..=(x_max as i32);
        let y_range = (part.coord.y as i32 - 1)..=(part.coord.y as i32 + 1);

        self.symbols.iter().any(|symbol| {
            x_range.contains(&(symbol.x as i32)) && y_range.contains(&(symbol.y as i32))
        })
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "#{}({}, {})",
            self.number, self.coord.x, self.coord.y
        ))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_to_string("data/day03.txt").expect("file");
    let lines = contents.lines();
    println!("Result: {}", process(lines)?);
    Ok(())
}

fn parse(lines: Lines) -> Result<Schematic, Box<dyn Error>> {
    let mut parts = Vec::new();
    let mut symbols = Vec::new();

    for (y, line) in lines.enumerate() {
        let mut chars_iter = line.trim().char_indices().peekable();
        while let Some((x, c)) = chars_iter.next() {
            match c {
                '.' => (),
                '0'..='9' => {
                    let mut number = c.to_digit(10).expect("digit");
                    while let Some((_, peek_char)) = chars_iter.peek() {
                        if peek_char.is_ascii_digit() {
                            number = number * 10 + peek_char.to_digit(10).expect("digit");
                            chars_iter.next();
                        } else {
                            break;
                        }
                    }
                    parts.push(Part {
                        coord: Coord::new(x, y),
                        number,
                    });
                }
                _ => {
                    symbols.push(Coord { x, y });
                }
            }
        }
    }

    Ok(Schematic { parts, symbols })
}

fn process(lines: Lines) -> Result<u32, Box<dyn Error>> {
    let schematic = parse(lines)?;
    Ok(schematic
        .parts
        .iter()
        .filter(|part| schematic.any_symbol_touches(part))
        .map(|part| part.number)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "467..114..
         ...*......
         ..35..633.
         ......#...
         617*......
         .....+.58.
         ..592.....
         ......755.
         ...$.*....
         .664.598..";

    #[test]
    fn example_answer() {
        let lines = EXAMPLE.lines();
        let answer = process(lines).expect("schematic");
        assert_eq!(answer, 4361);
    }

    #[test]
    fn example_parse() {
        let lines = EXAMPLE.lines();
        let schematic = parse(lines).expect("schematic");
        assert_eq!(
            schematic.parts.first(),
            Some(&Part {
                coord: Coord::new(0, 0),
                number: 467
            })
        );
        assert_eq!(schematic.symbols.first(), Some(&Coord::new(3, 1)));
    }

    #[test]
    fn test_answer() {
        let lines = "153..\n....*".lines();
        let answer = process(lines).expect("schematic");
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_parse() {
        let lines = "123".lines();
        let schematic = parse(lines).expect("schematic");
        assert_eq!(
            schematic.parts.first(),
            Some(&Part {
                coord: Coord::new(0, 0),
                number: 123
            })
        );

        let lines = "153..\n....*".lines();
        let schematic = parse(lines).expect("schematic");
        assert_eq!(
            schematic.parts.first(),
            Some(&Part {
                coord: Coord::new(0, 0),
                number: 153
            })
        );
        assert_eq!(schematic.symbols.first(), Some(&Coord::new(4, 1)));
    }

    #[test]
    fn touching() {
        let lines = "153..\n....*".lines();
        let schematic = parse(lines).expect("schematic");
        let part = schematic.parts.first().expect("part");
        assert!(!schematic.any_symbol_touches(part));
    }
}
