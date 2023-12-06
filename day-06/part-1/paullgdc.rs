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

fn run(input: &str) -> isize {
    let mut t = Tokenizer::new(input);

    let mut times = Vec::with_capacity(4);
    t.consume_fixed("Time:").unwrap();
    while t.curr_char() != Some(b'\n') {
        t.consume_whitespaces();
        times.push(t.consume_u32().unwrap());
    }

    let mut distances = Vec::with_capacity(4);
    t.consume_fixed("\nDistance:").unwrap();
    while t.curr_char().is_some() && t.curr_char() != Some(b'\n') {
        t.consume_whitespaces();
        distances.push(t.consume_u32().unwrap());
    }
    let mut tot = 1;
    for (&t, &d) in times.iter().zip(distances.iter()) {
        tot *= match resolve_poly(&[-(d as f64), t as f64, -1.0]) {
            Poly2Roots::NoRoots => 0,
            Poly2Roots::OneRoot(_) => 0,
            Poly2Roots::TwoRoots(lower, higer) => {
                ((higer - 1.0).ceil() as isize).saturating_sub((lower + 1.0).floor() as isize) + 1
            }
        };
    }
    tot
}

enum Poly2Roots {
    NoRoots,
    OneRoot(f64),
    TwoRoots(f64, f64),
}

fn resolve_poly(poly: &[f64; 3]) -> Poly2Roots {
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
            288
        )
    }
}
