use std::env::args;
use std::time::Instant;

use num::integer::Roots;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> usize {
    // Your code goes here
    let times: Vec<f32> = input
        .lines()
        .nth(0)
        .unwrap()
        .trim_start_matches("Time:")
        .split_ascii_whitespace()
        .map(|t| t.parse().unwrap())
        .collect::<Vec<f32>>();

    let records: Vec<f32> = input
        .lines()
        .nth(1)
        .unwrap()
        .trim_start_matches("Distance:")
        .split_ascii_whitespace()
        .map(|t| t.parse().unwrap())
        .collect::<Vec<f32>>();

    let mut res = 1;
    for i in 0..times.len() {
        let t1: f32 = (times[i] / 2.0 - (times[i] * times[i] / 4.0 - records[i] - 1.0).sqrt()).ceil();
        let t2: f32 = (times[i] / 2.0 + (times[i] * times[i] / 4.0 - records[i] - 1.0).sqrt()).floor();
        res *= (t2 - t1 + 1.0) as usize;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("Time:      7  15   30
Distance:  9  40  200"),
            288
        )
    }
}
