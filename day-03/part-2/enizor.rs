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

#[derive(Debug, Default)]
struct Gear {
    pos: i16,
    parts_count: u16,
    value: u32,
}

impl Gear {
    fn add_part(&mut self, part: u32) {
        self.parts_count += 1;
        self.value *= part;
    }
    fn ratio(&self) -> u32 {
        if self.parts_count == 2 {
            self.value
        } else {
            0
        }
    }
}

fn run(input: &str) -> u32 {
    let bytes = input.as_bytes().iter();
    let mut res = 0;
    let mut line_cur = 0;
    let mut numbers_prev_line = VecDeque::new();
    let mut numbers_current_line = VecDeque::new();
    let mut gears_prev_line: VecDeque<Gear> = VecDeque::new();
    let mut gears_current_line: VecDeque<Gear> = VecDeque::new();
    // Current parsed number
    let mut number = 0;
    // Current parsed number size, flags that number parsing is in progress
    let mut number_size = 0;
    // Start position in the line of the number
    let mut number_start = 0;
    // A gear was recently matched, used to process numbers on the same line as the gear
    let mut just_found_gear = false;
    // A number was recently matched, used to process gears on the same line as the number
    let mut just_found_number: bool = false;
    for &b in bytes.chain(b"\n\n".iter()) {
        // Parsing numbers takes priority
        if b.is_ascii_digit() {
            number *= 10;
            number += (b - b'0') as u32;
            if number_size == 0 {
                number_start = line_cur;
            }
            number_size += 1;
            just_found_number = true;
        } else {
            if number_size != 0 {
                // store for next-line matching
                numbers_current_line.push_back((number, number_start, number_start + number_size));
                // try to match with a gear on the previous line
                while let Some(gear) = gears_prev_line.front_mut() {
                    if gear.pos + 1 < number_start {
                        // the gear is too much to the left, it cannot match any number anymore
                        res += gear.ratio();
                        gears_prev_line.pop_front();
                        continue;
                    } else if gear.pos <= number_start + number_size {
                        // The gear is near the current number
                        gear.add_part(number);
                    }
                    // all remaining gears from the previous line are too much to the right, we'll have to match with a gear on the next line
                    break;
                }
                // try to match with a gear on the same line - it must be the last one parsed
                if just_found_gear {
                    gears_current_line.back_mut().unwrap().add_part(number);
                }
                number = 0;
                number_size = 0;
                number_start = 0;
            }
            if b == b'.' {
                just_found_gear = false;
                just_found_number = false;
            } else if b == b'\n' {
                // new line !
                just_found_gear = false;
                just_found_number = false;
                line_cur = 0;
                std::mem::swap(&mut gears_prev_line, &mut gears_current_line);
                std::mem::swap(&mut numbers_prev_line, &mut numbers_current_line);
                numbers_current_line.clear();
                for gear in &gears_current_line {
                    res += gear.ratio();
                }
                gears_current_line.clear();
                continue;
            } else if b == b'*' {
                // Found a gear !
                let mut gear = Gear {
                    pos: line_cur,
                    parts_count: 0,
                    value: 1,
                };
                if just_found_number {
                    gear.add_part(numbers_current_line.back().unwrap().0)
                }
                while let Some(&(val, start, end)) = numbers_prev_line.front() {
                    if line_cur + 1 >= start {
                        // last possibility for the number to match
                        numbers_prev_line.pop_front();
                        if line_cur <= end {
                            // The number is near the current gear
                            gear.add_part(val);
                        }
                    } else {
                        // all numbers are too much to the right
                        break;
                    }
                }
                // Store the gear position to match number on the next line
                gears_current_line.push_back(gear);
                just_found_gear = true;
                just_found_number = false;
            } else {
                // ignore rest of the symbols
                just_found_gear = false;
                just_found_number = false;
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
            467835
        )
    }
    #[test]
    fn same_line() {
        assert_eq!(
            run("
..2.3
.*11*"),
            55
        )
    }
}
