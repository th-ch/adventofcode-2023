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
    let mut res = 0;
    let mut first = true;
    let mut last_val = 0;
    for &c in input.as_bytes() {
        if c == b'\n' {
            res += last_val;
            first = true;
            last_val = 0;
        } else if c > b'0' && c <= b'9' {
            last_val = (c - b'0') as isize;
            if first {
                res += 10*last_val;
                first = false;
            }
        }
    }
    // ensure it works even if no trailing \n
    res + last_val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"), 142)
    }
}
