use std::env::args;
use std::time::Instant;

#[cfg(test)]
const CARD_ID_SIZE: usize = 8;

#[cfg(not(test))]
const CARD_ID_SIZE: usize = 10;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> isize {
    let input = input.as_bytes();
    let mut idx: usize = CARD_ID_SIZE;
    let mut acc: u8 = 0;
    let mut winning_numbers: u128 = 0;
    let mut scratching_phase = false;
    let mut current_card_score = 1;
    let mut total_score = 0;
    while idx < input.len() {
        match unsafe { input.get_unchecked(idx) } {
            c @ b'0'..=b'9' => {
                acc = acc * 10 + (c - b'0');
            }
            b' ' => {
                if acc != 0 {
                    if scratching_phase {
                        if winning_numbers & (1 << acc) != 0 {
                            current_card_score <<= 1;
                        }
                    } else {
                        winning_numbers |= 1 << acc;
                    }
                    acc = 0;
                }
            }
            b'|' => {
                scratching_phase = true;
            }
            b'\n' => {
                if winning_numbers & (1 << acc) != 0 {
                    total_score += current_card_score;
                } else {
                    total_score += current_card_score >> 1;
                }
                current_card_score = 1;
                acc = 0;
                winning_numbers = 0;
                scratching_phase = false;
                idx += CARD_ID_SIZE;
                continue;
            }
            c => panic!("unreachable: {:?}", std::str::from_utf8(&[*c])),
        }
        idx += 1;
    }
    if winning_numbers & (1 << acc) != 0 {
        total_score += current_card_score;
    } else {
        total_score += current_card_score >> 1;
    }
    total_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run(r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
            13
        )
    }
}
