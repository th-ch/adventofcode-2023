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
    let mut adjacents = Grid::new_from_vec(
        (row_len, column_len),
        vec![(1_u32, 0_u8); row_len * column_len],
    );
    for (j, l) in input.lines().enumerate() {
        if l.is_empty() {
            continue;
        }
        let mut number_span = None;
        for i in 0..l.len() {
            if l.as_bytes()[i].is_ascii_digit() {
                match number_span {
                    None => number_span = Some((i, i + 1)),
                    Some((_start, ref mut end)) => {
                        *end += 1;
                    }
                }
            } else if let Some(span) = number_span {
                let n = l[span.0..span.1].parse::<u32>().unwrap();
                for v in j.saturating_add_signed(-1)..=j.saturating_add_signed(1) {
                    for u in span.0.saturating_add_signed(-1)..=span.1 {
                        if v >= column_len || u >= row_len {
                            continue;
                        }
                        let c = input.as_bytes()[v * (row_len + 1) + u];
                        if c != b'.' && !c.is_ascii_digit() {
                            let Some((val, gears)) = adjacents.get_mut((u, v)) else {
                                continue;
                            };
                            *val *= n;
                            *gears += 1;
                        }
                    }
                }
                number_span = None;
            }
        }
        if let Some(span) = number_span {
            let n = l[span.0..span.1].parse::<u32>().unwrap();
            for v in j.saturating_add_signed(-1)..=j.saturating_add_signed(1) {
                for u in span.0.saturating_add_signed(-1)..=span.1 {
                    if v >= column_len || u >= row_len {
                        continue;
                    }
                    let c = input.as_bytes()[v * (row_len + 1) + u];
                    if c != b'.' && !c.is_ascii_digit() {
                        let Some((val, gears)) = adjacents.get_mut((u, v)) else {
                            continue;
                        };
                        *val *= n;
                        *gears += 1;
                    }
                }
            }
        }
    }
    let mut total = 0;
    for j in 0..adjacents.dims().1 {
        for i in 0..adjacents.dims().0 {
            if adjacents.get((i, j)).unwrap().1 == 2 {
                total += adjacents.get((i, j)).unwrap().0 as isize;
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
            467835
        )
    }
}
