use std::collections::VecDeque;
use std::env::args;
use std::ops::RangeInclusive;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

#[derive(Debug)]
struct Number {
    value: u32,
    positions: (i32, RangeInclusive<i32>),
}

#[derive(Debug)]
struct Pos {
    row: i32,
    col: i32,
}

fn run(input: &str) -> u32 {
    // Your code goes here
    // INIT
    let mut numbers = VecDeque::with_capacity(1000);
    let mut symbols = VecDeque::with_capacity(200);
    // PARSING
    let mut row = 0;
    let mut col = 0;
    let mut start_col = 0;
    let mut acc = 0;
    for c in input.as_bytes() {
        match c {
            b'0'..=b'9' => {
                if acc == 0 {
                    start_col = col;
                }
                acc = acc * 10 + (c - b'0') as u32;
            }
            b'\n' => {
                if acc != 0 {
                    numbers.push_back(Number {
                        value: acc,
                        positions: (row, start_col - 1..=col),
                    });
                    acc = 0;
                }
                row += 1;
                col = 0;
                continue;
            }
            b'.' => {
                if acc != 0 {
                    numbers.push_back(Number {
                        value: acc,
                        positions: (row, start_col - 1..=col),
                    });
                    acc = 0;
                }
            }
            _ => {
                if acc != 0 {
                    numbers.push_back(Number {
                        value: acc,
                        positions: (row, start_col - 1..=col),
                    });
                    acc = 0;
                }
                symbols.push_back(Pos { row, col });
            }
        }
        col += 1;
    }
    if acc != 0 {
        numbers.push_back(Number {
            value: acc,
            positions: (row, start_col - 1..=col),
        });
    }
    // ANALYSIS
    let mut res = 0;
    for number in numbers {
        while symbols
            .front()
            .is_some_and(|x| x.row < number.positions.0 - 1)
        {
            symbols.pop_front();
        }
        for symbol in &symbols {
            if symbol.row - 1 <= number.positions.0
                && number.positions.0 <= symbol.row + 1
                && number.positions.1.contains(&symbol.col)
            {
                res += number.value;
            }
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
            run(r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."),
            4361
        )
    }
}
