use std::error::Error;
use std::fs::read_to_string;
use std::io::Error as IoError;
use std::str::Lines;

type Id = usize;

type Map = Vec<Mapping>;

struct Mapping {
    source_start: Id,
    destination_start: Id,
    count: usize,
}

impl Mapping {
    // Clippy doesn't realize we're avoiding underflow
    #[allow(clippy::unnecessary_lazy_evaluations)]
    fn destination(&self, location: Id) -> Option<Id> {
        (location >= self.source_start && location < self.source_start + self.count)
            .then(|| (location - self.source_start) + self.destination_start)
    }
}

struct Almanac {
    seeds: Vec<Id>,

    /// Assumes maps are in order. If a source doesn't exist then it's destination is
    /// the same value.
    maps: Vec<Map>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_to_string("data/day05.txt").expect("file");
    let lines = contents.lines();
    let answer = answer(lines)?;
    println!("Answer: {answer}");
    Ok(())
}

fn parse(lines: Lines) -> Result<Almanac, Box<dyn Error>> {
    let mut lines = lines.peekable();

    // First line is seeds.
    let mut seeds = lines.next().expect("seeds line").split_whitespace();
    seeds.next();
    let seeds: Vec<Id> = seeds.map(|id| id.parse::<Id>().expect("id")).collect();
    lines.next().expect("blank line");

    // Categories
    let mut maps = Vec::new();
    while let Some(map_id_line) = lines.next() {
        if !map_id_line.ends_with(" map:") {
            return Err(IoError::other(format!(
                "Map name line '{map_id_line}' did not end with 'map:'"
            ))
            .into());
        }
        let _map_id = map_id_line
            .split_whitespace()
            .next()
            .expect("map id")
            .to_owned();

        let mut mappings = Vec::new();

        while let Some(category_line) = lines.next() {
            // Destination, start, count
            let mut line = category_line
                .split_whitespace()
                .map(|n| n.parse::<Id>().expect("number"));
            mappings.push(Mapping {
                destination_start: line.next().expect("destination"),
                source_start: line.next().expect("source"),
                count: line.next().expect("count"),
            });

            // End the category on a blank line.
            if let Some(next_line) = lines.peek() {
                if next_line.is_empty() {
                    lines.next();
                    break;
                }
            }
        }

        maps.push(mappings);
    }

    Ok(Almanac { seeds, maps })
}

fn answer(lines: Lines) -> Result<Id, Box<dyn Error>> {
    let almanac = parse(lines)?;

    // Keep track of the path for debugging.
    let paths: Vec<Vec<Id>> = almanac
        .seeds
        .iter()
        .map(|seed| path(*seed, &almanac.maps))
        .collect();

    Ok(*paths
        .iter()
        .map(|path| path.last().unwrap())
        .min()
        .expect("at least one element"))
}

fn path(start: Id, maps: &Vec<Map>) -> Vec<Id> {
    // FIXME: Fold
    let mut path = Vec::new();
    path.push(start);
    for map in maps {
        let location = path.last().unwrap();
        let dest = map
            .iter()
            .find_map(|mapping| mapping.destination(*location))
            .unwrap_or(*location);
        path.push(dest);
    }
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_answer() {
        let contents = read_to_string("data/day05example.txt").expect("file");
        let lines = contents.lines();
        let answer = answer(lines).expect("answer");
        assert_eq!(answer, 35);
    }
}
