use std::collections::VecDeque;
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
    let bytes = input.as_bytes().iter();
    let mut res = 0;
    let mut line_cur = 0;
    let mut numbers_prev_line = VecDeque::new();
    let mut symbols_prev_line = VecDeque::new();
    let mut numbers_current_line = VecDeque::new();
    let mut symbols_current_line = VecDeque::new();
    // Current parsed number
    let mut number = 0;
    // Current parsed number size, flags that number parsing is in progress
    let mut number_size = 0;
    // Start position in the line of the number
    let mut number_start = 0;
    // A symbol was recently matched, used to process numbers on the same line as the symbol
    let mut just_found_symbol = false;
    for &b in bytes.chain(b"\n".iter()) {
        // Parsing numbers takes priority
        if b.is_ascii_digit() {
            number *= 10;
            number += (b - b'0') as usize;
            if number_size == 0 {
                number_start = line_cur;
            }
            number_size += 1;
        } else {
            if number_size != 0 {
                if just_found_symbol || (b != b'.' && b != b'\n') {
                    res += number;
                } else {
                    // store for next-line matching
                    numbers_current_line.push_back((
                        number,
                        number_start,
                        number_start + number_size,
                    ));
                    // try to match with a symbol on the previous line
                    while let Some(&cur) = symbols_prev_line.front() {
                        if cur + 1 < number_start {
                            // the symbol is too much to the left, it cannot match any number anymore
                            symbols_prev_line.pop_front();
                            continue;
                        } else if cur <= number_start + number_size {
                            // The symbol is near the current number
                            res += number;
                            // prevent double matching
                            numbers_current_line.pop_back();
                        }
                        // all remaining symbols from the previous line are too much to the right, we'll have to match with a symbol on the next line
                        break;
                    }
                }
                number = 0;
                number_size = 0;
                number_start = 0;
            }
            if b == b'.' {
                just_found_symbol = false;
            } else if b == b'\n' {
                // new line !
                just_found_symbol = false;
                line_cur = 0;
                std::mem::swap(&mut symbols_prev_line, &mut symbols_current_line);
                std::mem::swap(&mut numbers_prev_line, &mut numbers_current_line);
                symbols_current_line.clear();
                numbers_current_line.clear();
                continue;
            } else {
                // Found a symbol !
                while let Some(&(val, start, end)) = numbers_prev_line.front() {
                    if line_cur + 1 >= start {
                        // last possibility for the number to match
                        numbers_prev_line.pop_front();
                        if line_cur <= end {
                            // The number is near the current symbol
                            res += val;
                        }
                    } else {
                        // all numbers are too much to the right
                        break;
                    }
                }
                // Store the symbol position to match number on the next line
                symbols_current_line.push_back(line_cur);
                just_found_symbol = true;
            }
        }
        line_cur += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("
467..114..
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
