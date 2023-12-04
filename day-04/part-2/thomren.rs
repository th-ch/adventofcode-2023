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

fn run(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut scores = vec![1; lines.len()];
    for i in (0..lines.len()).rev() {
        let score = count_winning_numbers(lines[i]);
        for j in 1..(score + 1) {
            if i + j >= scores.len() {
                break;
            }
            scores[i] += scores[i + j];
        }
    }
    scores.iter().sum()
}

fn count_winning_numbers(card: &str) -> usize {
    let (_, numbers) = card.split_once(": ").unwrap();
    let (winning_numbers, my_numbers) = numbers.split_once(" | ").unwrap();
    let winning_numbers: HashSet<usize> = winning_numbers
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    my_numbers
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .filter(|n| winning_numbers.contains(n))
        .count()
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
            30
        )
    }
}
