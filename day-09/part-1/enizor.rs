use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn predict(seq: &mut [isize]) -> isize {
    let mut len = seq.len();
    if len == 0 {
        return 0;
    }
    let mut zeroes = 0;
    let mut res = 0;
    while zeroes < len {
        zeroes = 0;
        len -= 1;
        res += seq[len];
        for i in 0..len {
            seq[i] = seq[i + 1] - seq[i];
            if seq[i] == 0 {
                zeroes += 1;
            }
        }
    }
    res
}

fn run(input: &str) -> isize {
    let mut res = 0;
    for line in input.lines() {
        let mut parsed = line
            .split_ascii_whitespace()
            .map(|w| w.parse().expect("failed to parse!"))
            .collect::<Vec<_>>();
        res += predict(&mut parsed);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("0 3 6 9 12 15"), 18);
        assert_eq!(run("1 3 6 10 15 21"), 28);
        assert_eq!(run("10 13 16 21 30 45"), 68);
        assert_eq!(run("-5 0  4  7   9 10"), 10);
    }
}
