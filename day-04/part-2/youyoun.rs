use std::collections::{HashMap, HashSet};
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
    let mut card_count: HashMap<usize, usize> = HashMap::new();
    let mut n_copies: usize = 0;
    for (i, line) in input
        .lines()
        .map(|line: &str| line.split(":").nth(1).unwrap().trim())
        .enumerate()
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
        let n_winning_numbers: usize =
            winning_numbers.intersection(&scratched_numbers).count();
        
        if card_count.contains_key(&(i+1)) {
            card_count.insert(i+1, card_count[&(i+1)] + 1);
        } else {
            card_count.insert(i+1, 1);
        }
        if n_winning_numbers > 0 {
            if n_copies == 0 {
                n_copies = n_winning_numbers;
                for j in i+2..n_winning_numbers+i+2 {
                    card_count.insert(j, 1);
                }
            } else {
                for j in i+2..n_winning_numbers+i+2 {
                    card_count.insert(j, card_count[&j] + 1*card_count[&(i+1)]);
                }
            }
        }
        if n_copies > 0 {
            n_copies -= 1;
        }
        // println!("{}", i+1);
        // println!("{:?}", n_winning_numbers);
        // println!("{:?}", card_count);
    }
    card_count.values().sum()
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
