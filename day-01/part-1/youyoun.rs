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
    let mut s: u32 = 0;
    for line in input.lines() {
        let mut is_first_set: bool = false;
        let mut first: u32 = 0;
        let mut last: u32 = 0;
        for c in line.chars() {
            if c.is_ascii_digit() {
                let digit: u32 = c.to_digit(10).unwrap();

                if !is_first_set {
                    first = digit;
                    is_first_set = true;
                }
                last = digit;
            }
        }
        s += first * 10 + last;
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("Test example"), 0)
    }
}
