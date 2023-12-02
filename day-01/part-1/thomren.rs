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
    let (mut first, mut last) = (0, 0);
    let bytes  = line.as_bytes();
    for &byte in bytes.iter() {
        if (b'0'..=b'9').contains(&byte) {
            first = (byte - b'0') as usize;
            break;
        }
    }
    for &byte in bytes.iter().rev() {
        if (b'0'..=b'9').contains(&byte) {
            last = (byte - b'0') as usize;
            break;
        }
    }
    first * 10 + last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"),
            142
        )
    }
}
