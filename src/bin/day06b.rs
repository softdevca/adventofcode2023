use std::error::Error;
use std::fs::read_to_string;
use std::str::Lines;

struct Race {
    time_limit: u64,
    record_distance: u64,
}

impl Race {
    /// Charge times that result in a win.
    fn winning_times(&self) -> Vec<u64> {
        (0..self.time_limit)
            .filter(|charge_time| self.distance(*charge_time) > self.record_distance)
            .collect()
    }

    fn distance(&self, charge_time: u64) -> u64 {
        let velocity = charge_time;
        (self.time_limit - charge_time) * velocity
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_to_string("data/day06.txt").expect("file");
    let lines = contents.lines();
    let answer = answer(lines)?;
    println!("Answer: {answer}");
    Ok(())
}

fn parse(lines: Lines) -> Result<Vec<Race>, Box<dyn Error>> {
    let mut number_lines = lines.map(|line| {
        line.split_once(':')
            .expect("line with colon")
            .1
            .replace(' ', "")
            .parse::<u64>()
            .expect("number")
    });
    let time_limit = number_lines.next().unwrap();
    let record_distance = number_lines.next().unwrap();
    Ok(vec![Race {
        time_limit,
        record_distance,
    }])
}

fn answer(lines: Lines) -> Result<u32, Box<dyn Error>> {
    let races = parse(lines)?;
    Ok(races
        .iter()
        .map(|race| race.winning_times().len() as u32)
        .product())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "Time:      7  15   30\nDistance:  9  40  200";

    #[test]
    fn example_answer() {
        let lines = EXAMPLE.lines();
        let answer = answer(lines).unwrap();
        assert_eq!(answer, 71503);
    }
}
