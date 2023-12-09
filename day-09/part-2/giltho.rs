use std::env::args;
use std::time::Instant;

#[cfg(test)]
const FACTORS: [isize; 6] = [6, -15, 20, -15, 6, -1];

#[cfg(not(test))]
const FACTORS: [isize; 21] = [
    21, -210, 1330, -5985, 20349, -54264, 116280, -203490, 293930, -352716, 352716, -293930,
    203490, -116280, 54264, -20349, 5985, -1330, 210, -21, 1,
];

fn main() {
    let now = Instant::now();
    let output = unsafe { run(&args().nth(1).expect("Please provide an input")) };
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

unsafe fn run(input: &str) -> isize {
    let mut res = 0;
    let mut idx = 0;
    let mut sign = 1;
    let mut acc = 0;
    for c in input.as_bytes() {
        match c {
            b'-' => sign = -1,
            b' ' => {
                res += sign * acc * FACTORS[idx];
                idx += 1;
                sign = 1;
                acc = 0;
            }
            b'\n' => {
                res += sign * acc * FACTORS[idx];
                idx = 0;
                sign = 1;
                acc = 0;
            }
            _ => {
                acc = acc * 10 + (c - b'0') as isize;
            }
        }
    }
    res + sign * acc * FACTORS[idx]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            unsafe {
                run(r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45")
            },
            2
        )
    }
}
