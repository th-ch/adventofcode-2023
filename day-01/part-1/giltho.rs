use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> isize {
    let mut sum = 0;
    for line in input.as_bytes().split(|x| *x == b'\n') {
        let mut first = -1;
        let mut last = -1;
        for c in line {
            match c {
                b'0'..=b'9' => {
                    last = (*c - b'0') as isize;
                    if first == -1 {
                        first = last;
                    }
                }
                _ => continue,
            }
        }
        sum += first * 10 + last;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run(r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"),
            142
        )
    }
}
