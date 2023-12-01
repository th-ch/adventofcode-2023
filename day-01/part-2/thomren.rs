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
    input.lines().map(get_calibration_value).sum()
}

fn get_calibration_value(line: &str) -> usize {
    let values = parse_numbers(line);
    if values.len() == 0 {
        return 0;
    }
    return *values.first().unwrap() * 10 + *values.last().unwrap();
}

fn parse_numbers(line: &str) -> Vec<usize> {
    let mut numbers = vec![];
    let chars = line.chars().collect::<Vec<char>>();
    for i in 0..chars.len() {
        if chars[i] >= '0' && chars[i] <= '9' {
            numbers.push(chars[i] as usize - '0' as usize);
        } else {
            let suffix = chars[i..chars.len()].iter().collect::<String>();
            if suffix.starts_with("one") {
                numbers.push(1);
            } else if suffix.starts_with("two") {
                numbers.push(2);
            } else if suffix.starts_with("three") {
                numbers.push(3);
            } else if suffix.starts_with("four") {
                numbers.push(4);
            } else if suffix.starts_with("five") {
                numbers.push(5);
            } else if suffix.starts_with("six") {
                numbers.push(6);
            } else if suffix.starts_with("seven") {
                numbers.push(7);
            } else if suffix.starts_with("eight") {
                numbers.push(8);
            } else if suffix.starts_with("nine") {
                numbers.push(9);
            }
        }
    }
    numbers
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
