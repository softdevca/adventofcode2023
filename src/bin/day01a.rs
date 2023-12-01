use std::error::Error;
use std::fs::read_to_string;
use std::str::Lines;

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_to_string("data/day01.txt").expect("file");
    let lines = contents.lines();
    println!("Total: {}", total(lines)?);
    Ok(())
}

fn total(lines: Lines) -> Result<u32, Box<dyn Error>> {
    let total = lines
        // Only digits
        .map(|line| line.chars().filter(|c| c.is_ascii_digit()))
        // First and last characters
        .map(|mut digits| {
            let first = digits
                .next()
                .expect("at least one digit")
                .to_digit(10)
                .unwrap();
            let last = digits.last().map_or(first, |n| n.to_digit(10).unwrap());
            (first, last)
        })
        // Digits to numbers
        .map(|(first, last)| first * 10 + last)
        .sum();
    Ok(total)
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn example() {
        let data = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";
        let total = total(data.lines()).unwrap();
        assert_eq!(total, 142);
    }
}
