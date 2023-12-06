use aoc::enizor::utils::beat_race;
use std::collections::VecDeque;
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
    let mut times = VecDeque::with_capacity(5);
    let mut parse_time = true;
    let mut res = 1;
    for w in input.split_ascii_whitespace().skip(1) {
        if let Ok(num) = w.parse() {
            if parse_time {
                times.push_back(num);
            } else {
                let t = times.pop_front().unwrap();
                res *= beat_race(t, num);
            }
        } else {
            parse_time = false;
        }
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
