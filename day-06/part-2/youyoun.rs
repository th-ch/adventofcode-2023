use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> i64 {
    // Your code goes here
    let times: f64 = input
        .lines()
        .nth(0)
        .unwrap()
        .trim_start_matches("Time:")
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap();

    let records: f64 = input
        .lines()
        .nth(1)
        .unwrap()
        .trim_start_matches("Distance:")
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap();

    let t1: f64 = (times / 2.0 - (times * times / 4.0 - records - 1.0).sqrt()).ceil();
    let t2: f64 = (times / 2.0 + (times * times / 4.0 - records - 1.0).sqrt()).floor();

    (t2 - t1) as i64 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("Time:      7  15   30
Distance:  9  40  200"),
            71503
        )
    }
}
