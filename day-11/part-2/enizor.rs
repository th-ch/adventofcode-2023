use std::env::args;
use std::time::Instant;

use aoc::enizor::bitset::*;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

#[derive(Debug, Default, Clone, Copy)]
struct Galaxy {
    x: usize,
    y: usize,
}

fn dist(a: &Galaxy, b: &Galaxy) -> usize {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

const EXPANSION_RATE: usize = 1000000;

fn run(input: &str) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut expand_x = ArrayBitSet::<{ bitset_size(256) }>::new();
    let mut galaxies = Vec::with_capacity(64);
    let mut empty_line = true;
    let mut max_x = 0;
    for b in input.bytes() {
        match b {
            b'#' => {
                galaxies.push(Galaxy { x, y });
                expand_x.set(x);
                empty_line = false;
                x += 1;
            }
            b'\n' => {
                max_x = max_x.max(x);
                x = 0;
                y += if empty_line { EXPANSION_RATE } else { 1 };
                empty_line = true;
            }
            _ => x += 1,
        }
    }
    let mut offset = vec![0; max_x + 1];
    let mut prev = 0;
    for (x, o) in offset.iter_mut().enumerate() {
        *o = prev;
        if !expand_x.test(x) {
            *o += EXPANSION_RATE - 1
        }
        prev = *o;
    }
    for g in &mut galaxies {
        g.x += offset[g.x];
    }
    let mut res = 0;
    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            res += dist(&galaxies[i], &galaxies[j]);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."),
            82000210
        )
    }
}
