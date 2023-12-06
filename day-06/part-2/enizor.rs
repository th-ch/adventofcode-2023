use aoc::enizor::utils::beat_race;
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
    let mut parsed = [0.; 2];
    let mut cur = 0;
    for b in input.as_bytes() {
        match *b {
            d if d.is_ascii_digit() => {
                parsed[cur] *= 10.;
                parsed[cur] += (d - b'0') as f64;
            }
            b'\n' => {
                cur += 1;
            }
            _ => {}
        }
    }
    beat_race(parsed[0], parsed[1])
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
