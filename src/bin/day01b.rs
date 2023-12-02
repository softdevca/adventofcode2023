use std::error::Error;
use std::fs::read_to_string;
use std::str::Lines;

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_to_string("data/day01.txt").expect("file");
    let lines = contents.lines();
    println!("Total: {}", total(lines)?);
    Ok(())
}

/// Find a possible digit for each character. The words may overlap.
fn line_to_digits(line: &str) -> Vec<u8> {
    let chars = line.chars().collect::<Vec<char>>();
    (0..chars.len())
        .filter_map(|start_pos| match chars[start_pos..] {
            ['0', ..] | ['z', 'e', 'r', 'o', ..] => Some(0),
            ['1', ..] | ['o', 'n', 'e', ..] => Some(1),
            ['2', ..] | ['t', 'w', 'o', ..] => Some(2),
            ['3', ..] | ['t', 'h', 'r', 'e', 'e', ..] => Some(3),
            ['4', ..] | ['f', 'o', 'u', 'r', ..] => Some(4),
            ['5', ..] | ['f', 'i', 'v', 'e', ..] => Some(5),
            ['6', ..] | ['s', 'i', 'x', ..] => Some(6),
            ['7', ..] | ['s', 'e', 'v', 'e', 'n', ..] => Some(7),
            ['8', ..] | ['e', 'i', 'g', 'h', 't', ..] => Some(8),
            ['9', ..] | ['n', 'i', 'n', 'e', ..] => Some(9),
            _ => None,
        })
        .collect::<Vec<u8>>()
}

fn total(lines: Lines) -> Result<u32, Box<dyn Error>> {
    let total = lines
        .map(line_to_digits)
        .map(|digits| {
            let first = digits.first().expect("at least one digit");
            let last = digits.last().map_or(first, |n| n);
            (first * 10 + last) as u32
        })
        .sum();
    Ok(total)
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn example() {
        let data = "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen";
        let total = total(data.lines()).unwrap();
        assert_eq!(total, 281);
    }
}
