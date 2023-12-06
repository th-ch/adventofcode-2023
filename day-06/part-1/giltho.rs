use std::env::args;
use std::time::Instant;

#[cfg(test)]
const RUNS: usize = 3;

#[cfg(not(test))]
const RUNS: usize = 4;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

#[allow(non_snake_case)]
#[inline(always)]
fn calc(tm: f64, b: f64) -> isize {
    let sqΔ = (tm * tm - 4. * (b + 1.)).sqrt();
    let t0 = (tm + sqΔ) / 2.;
    let t1 = (tm - sqΔ) / 2.;
    (t0.floor() - t1.ceil()) as isize + 1
}

fn run(input: &str) -> isize {
    let input = input.as_bytes();
    let mut data = [0f64; RUNS * 2];
    let mut i = 9;
    let mut j = 0;
    let mut acc: u32 = 0;
    while i < input.len() {
        match unsafe { input.get_unchecked(i) } {
            b' ' => {
                if acc != 0 {
                    data[j] = acc as f64;
                    j += 1;
                    acc = 0;
                }
            }
            b'\n' => {
                data[j] = acc as f64;
                j += 1;
                acc = 0;
                i += 9;
            }
            c @ b'0'..=b'9' => {
                acc = acc * 10 + ((c - b'0') as u32);
            }
            _ => unreachable!(),
        };
        i += 1;
    }
    data[j] = acc as f64;
    let mut res = 1;
    for i in 0..RUNS {
        res *= calc(data[i], data[i + RUNS]);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run(r"Time:      7  15   30
Distance:  9  40  200"),
            288
        )
    }
}
