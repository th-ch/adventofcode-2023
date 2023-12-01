use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

const SPELLED: [&[u8]; 9] = [
    "one".as_bytes(),
    "two".as_bytes(),
    "three".as_bytes(),
    "four".as_bytes(),
    "five".as_bytes(),
    "six".as_bytes(),
    "seven".as_bytes(),
    "eight".as_bytes(),
    "nine".as_bytes()
];

fn try_match(input: &[u8]) -> Option<isize> {
    let b = input[0];
    if b >= b'1' && b <= b'9' {
        Some((b - b'0') as isize)
    } else {
        for (i, t) in SPELLED.iter().enumerate() {
            if input.len() >= t.len() && &input[..t.len()] == *t {
                return Some((i + 1) as isize);
            }
        }
        None
    }
}

fn run(input: &str) -> isize {
    let mut res = 0;
    let bytes = input.as_bytes();
    let mut first = true;
    let mut last_val = 0;
    let mut cur = 0;
    while cur < bytes.len() {
        if bytes[cur] == b'\n' {
            res += last_val;
            first = true;
            last_val = 0;
        } else if let Some(val) = try_match(&bytes[cur..]) {
            last_val = val;
            if first {
                res += 10*last_val;
                first = false;
            }
        }
        cur += 1;
    }
    res + last_val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"), 281)
    }
}
