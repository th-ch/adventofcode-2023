use std::env::args;
use std::time::Instant;

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
    let mut time = 0f64;
    let mut i = 9;
    let mut acc: u64 = 0;
    while i < input.len() {
        match unsafe { input.get_unchecked(i) } {
            c @ b'0'..=b'9' => {
                acc = acc * 10 + ((c - b'0') as u64);
            }
            b'\n' => {
                time = acc as f64;
                acc = 0;
                i += 9;
            }
            _ => (),
        };
        i += 1;
    }
    calc(time, acc as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run(r"Time:      7  15   30
Distance:  9  40  200"),
            71503
        )
    }
}
