use std::collections::HashSet;
use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> u32 {
    // Your code goes here
    let mut n_points: u32 = 0;
    for line in input
        .lines()
        .map(|line: &str| line.split(":").nth(1).unwrap().trim())
    {
        let winning_numbers = line
            .split("|")
            .nth(0)
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let scratched_numbers = line
            .split("|")
            .nth(1)
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let n_winning_numbers: u32 = winning_numbers.intersection(&scratched_numbers).count() as u32;
        if n_winning_numbers > 0 {
            n_points += 2u32.pow(n_winning_numbers-1)
        }
    }
    n_points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
            13
        )
    }
}
