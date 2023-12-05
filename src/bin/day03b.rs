use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs::read_to_string;
use std::str::Lines;

#[derive(Eq, Hash, PartialEq)]
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

#[derive(Debug, Eq, Hash, PartialEq)]
struct Symbol {
    name: char,
    coord: Coord,
}

impl Symbol {
    fn is_gear(&self) -> bool {
        self.name == '*'
    }
}

struct Schematic {
    parts: Vec<Part>,
    symbols: Vec<Symbol>,
}

impl Schematic {
    fn symbols_adjacent(&self, part: &Part) -> Vec<&Symbol> {
        // Symbol must be in this range to touch.
        let x_min = part.coord.x;
        let x_max = x_min + part.number.to_string().len();
        let x_range = (x_min as i32 - 1)..=(x_max as i32);
        let y_range = (part.coord.y as i32 - 1)..=(part.coord.y as i32 + 1);

        self.symbols
            .iter()
            .filter(|symbol| {
                x_range.contains(&(symbol.coord.x as i32))
                    && y_range.contains(&(symbol.coord.y as i32))
            })
            .collect::<Vec<&Symbol>>()
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
                    symbols.push(Symbol {
                        name: c,
                        coord: Coord::new(x, y),
                    });
                }
            }
        }
    }

    Ok(Schematic { parts, symbols })
}

fn process(lines: Lines) -> Result<u32, Box<dyn Error>> {
    let schematic = parse(lines)?;
    let symbol_to_part = schematic
        .parts
        .iter()
        .flat_map(|part| {
            schematic
                .symbols_adjacent(part)
                .into_iter()
                .map(move |symbol| (symbol, part))
        })
        .filter(|(symbol, _)| symbol.is_gear());

    let mut gears_to_parts: HashMap<&Symbol, Vec<&Part>> = HashMap::new();
    for (symbol, part) in symbol_to_part {
        gears_to_parts
            .entry(symbol)
            .and_modify(|parts| parts.push(part))
            .or_insert(vec![part]);
    }

    // Gear ratios
    let ratios = gears_to_parts.into_iter().filter_map(|(_, parts)| {
        if parts.len() == 2 {
            Some(parts.first().unwrap().number * parts.last().unwrap().number)
        } else {
            None
        }
    });

    Ok(ratios.sum())
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
        assert_eq!(answer, 467835);
    }

    #[test]
    fn test_answer() {
        let lines = "100.200.\n...*....".lines();
        let answer = process(lines).expect("schematic");
        assert_eq!(answer, 100 * 200);
    }
}
