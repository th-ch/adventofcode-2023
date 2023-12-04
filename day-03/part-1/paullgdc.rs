use std::env::args;
use std::time::Instant;

use aoc::paullgdc::grid::Grid;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> isize {
    let row_len = input.lines().next().unwrap().len();
    let column_len = (input.len() + 1) / (row_len + 1);
    let mut adjacents =
        Grid::new_from_vec((row_len, column_len), vec![false; row_len * column_len]);

    for (j, l) in input.lines().enumerate() {
        if l.is_empty() {
            continue;
        }
        for i in 0..l.len() {
            let c = l.as_bytes()[i];
            if c == b'.' || c.is_ascii_digit() {
                continue;
            }
            for a in [-1, 0, 1] {
                for b in [-1, 0, 1] {
                    let idx = (i.wrapping_add_signed(a), j.wrapping_add_signed(b));
                    if let Some(p) = adjacents.get_mut(idx) {
                        *p = true;
                    }
                }
            }
        }
    }

    let mut total = 0;
    for (j, l) in input.lines().enumerate() {
        if l.is_empty() {
            continue;
        }
        let mut number_span = None;
        let mut is_part_number = false;
        for i in 0..l.len() {
            if l.as_bytes()[i].is_ascii_digit() {
                if adjacents.get((i, j)).copied() == Some(true) {
                    is_part_number = true;
                }
                match number_span {
                    None => number_span = Some((i, i + 1)),
                    Some((_start, ref mut end)) => {
                        *end += 1;
                    }
                }
            } else if let Some(span) = number_span {
                if is_part_number {
                    total += l[span.0..span.1].parse::<isize>().unwrap();
                }
                is_part_number = false;
                number_span = None;
            } else {
                is_part_number = false;
            }
        }
        if let Some(span) = number_span {
            if is_part_number {
                total += l[span.0..span.1].parse::<isize>().unwrap();
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("467..114..
...*......
..35..633.
......#...
617-......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"),
            4361
        )
    }
}
