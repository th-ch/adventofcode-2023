use std::env::args;
use std::time::Instant;

use aoc::paullgdc::tokenizer::Tokenizer;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

struct Race {
    time: u32,
    record: u32,
}

fn run(input: &str) -> isize {
    let mut t = Tokenizer::new(input);

    let mut time: u64 = 0;
    t.consume_fixed("Time:").unwrap();
    while t.curr_char() != Some(b'\n') {
        t.consume_whitespaces();
        let part = t.consume_numeric();
        for b in part.as_bytes() {
            time *= 10;
            time += (b - b'0') as u64;
        }
    }

    let mut distance: u64 = 0;
    t.consume_fixed("\nDistance:").unwrap();
    while t.curr_char().is_some() && t.curr_char() != Some(b'\n') {
        t.consume_whitespaces();
        let part = t.consume_numeric();
        for b in part.as_bytes() {
            distance *= 10;
            distance += (b - b'0') as u64;
        }
    }

    match resolve_poly([-(distance as f64), time as f64, -1.0]) {
        Poly2Roots::NoRoots => 0,
        Poly2Roots::OneRoot(_) => 0,
        Poly2Roots::TwoRoots(lower, higer) => {
            ((higer - 1.0).ceil() as isize).saturating_sub((lower + 1.0).floor() as isize) + 1
        }
    }
}

enum Poly2Roots {
    NoRoots,
    OneRoot(f64),
    TwoRoots(f64, f64),
}

fn resolve_poly(poly: [f64; 3]) -> Poly2Roots {
    let det = poly[1] * poly[1] - 4.0 * poly[0] * poly[2];
    match det.total_cmp(&0.0) {
        std::cmp::Ordering::Greater => {
            if poly[2] > 0.0 {
                Poly2Roots::TwoRoots(
                    (-poly[1] - det.sqrt()) / (2.0 * poly[2]),
                    (-poly[1] + det.sqrt()) / (2.0 * poly[2]),
                )
            } else {
                Poly2Roots::TwoRoots(
                    (-poly[1] + det.sqrt()) / (2.0 * poly[2]),
                    (-poly[1] - det.sqrt()) / (2.0 * poly[2]),
                )
            }
        }
        std::cmp::Ordering::Equal => Poly2Roots::OneRoot(-poly[1] / (2.0 * poly[2])),
        std::cmp::Ordering::Less => Poly2Roots::NoRoots,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("Time:      7  15   30
Distance:  9  40  200
"),
            71503
        )
    }
}
