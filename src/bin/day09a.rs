use std::error::Error;
use std::fs::read_to_string;
use std::str::Lines;

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_to_string("data/day09.txt").expect("file");
    let lines = contents.lines();
    let answer = answer(lines)?;
    println!("Answer: {answer}");
    Ok(())
}

fn parse(lines: Lines) -> Vec<Vec<i64>> {
    lines
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i64>().expect("number"))
                .collect()
        })
        .collect()
}

/// Difference each level until the last row is all zeros.
fn difference_table(history: &[i64]) -> Vec<Vec<i64>> {
    let mut table: Vec<Vec<i64>> = Vec::new();
    table.push(history.to_vec());
    while table.last().unwrap().iter().any(|n| *n != 0) {
        let differences = table
            .last()
            .unwrap()
            .windows(2)
            .map(|a| a[1] - a[0])
            .collect::<Vec<i64>>();
        table.push(differences);
    }
    table
}

fn extrapolate(history: &Vec<i64>) -> Vec<i64> {
    let differences = difference_table(history);
    let mut extrapolations = Vec::with_capacity(history.len());
    extrapolations.push(0);
    for row in differences.iter().rev() {
        let previous = extrapolations.last().unwrap();
        extrapolations.push(previous + row.last().unwrap());
    }
    extrapolations
}

fn answer(lines: Lines) -> Result<i64, Box<dyn Error>> {
    let histories = parse(lines);
    let extrapolated = histories.iter().map(extrapolate).collect::<Vec<Vec<i64>>>();
    Ok(extrapolated.iter().map(|t| t.last().unwrap()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";

    #[test]
    fn example_answer() {
        let lines = EXAMPLE.lines();
        let answer = answer(lines).unwrap();
        assert_eq!(answer, 114);
    }
}
