use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

static NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn run(input: &str) -> usize {
    input.lines().map(|line| get_first_number(line) * 10 + get_last_number(line)).sum()
}

fn get_first_number(line: &str) -> usize {
    let bytes = line.as_bytes();
    for i in 0..bytes.len() {
        if bytes[i] >= b'0' && bytes[i] <= b'9' {
            return (bytes[i] - b'0') as usize;
        } else {
            for j in 0..NUMBERS.len() {
                if bytes[i..].starts_with(NUMBERS[j].as_bytes()) {
                    return j as usize + 1;
                }
            }
        }
    }
    0
}

fn get_last_number(line:&str) -> usize {
    let bytes = line.as_bytes();
    for i in (0..bytes.len()).rev() {
        if bytes[i] >= b'0' && bytes[i] <= b'9' {
            return (bytes[i] - b'0') as usize;
        } else {
            for j in 0..NUMBERS.len() {
                if bytes[i..].starts_with(NUMBERS[j].as_bytes()) {
                    return  j as usize + 1;
                }
            }
        }
    }
    0
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
7pqrstsixteen"),
            281
        )
    }
}
