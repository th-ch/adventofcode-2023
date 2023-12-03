use std::env::args;
use std::time::Instant;

use aoc::paullgdc;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn number_str_to_value(number: &str) -> isize {
    match number {
        "0" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five"=> 5,
        "6"| "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine"=> 9,
        _ => 0
    }
}

fn run(input: &str) -> isize {
    let searcher = paullgdc::aho_corasik::AhoCorasik::new(vec![
        "0".to_owned(),
        "1".to_owned(),
        "one".to_owned(),
        "2".to_owned(),
        "two".to_owned(),
        "3".to_owned(),
        "three".to_owned(),
        "4".to_owned(),
        "four".to_owned(),
        "5".to_owned(),
        "five".to_owned(),
        "6".to_owned(),
        "six".to_owned(),
        "7".to_owned(),
        "seven".to_owned(),
        "8".to_owned(),
        "eight".to_owned(),
        "9".to_owned(),
        "nine".to_owned(),
    ]);
    input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut i = searcher.search(l.as_bytes());
            let (_, first) = i.next().unwrap();
            let (_, second) = i.last().unwrap_or((0, first));
            ((number_str_to_value(first)) * 10 + number_str_to_value(second)) as isize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen   
"),
            281
        )
    }
}
