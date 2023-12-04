use std::env::args;
use std::time::Instant;

#[cfg(test)]
const CARD_ID_SIZE: usize = 8;

#[cfg(not(test))]
const CARD_ID_SIZE: usize = 10;

#[cfg(test)]
const MAX_MATCHING_NUMBERS: usize = 5;

#[cfg(not(test))]
const MAX_MATCHING_NUMBERS: usize = 10;

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
    let mut current_card_wins = 0;
    let mut card_rot = 0;
    let mut future_cards = [1u32; MAX_MATCHING_NUMBERS + 1];
    let mut res = 0;
    while idx < input.len() {
        match unsafe { input.get_unchecked(idx) } {
            c @ b'0'..=b'9' => {
                acc = acc * 10 + (c - b'0');
            }
            b' ' => {
                if acc != 0 {
                    if scratching_phase {
                        if winning_numbers & (1 << acc) != 0 {
                            current_card_wins += 1;
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
                    current_card_wins += 1;
                }
                acc = 0;
                for i in 1..=current_card_wins {
                    future_cards[(card_rot + i) % (MAX_MATCHING_NUMBERS + 1)] +=
                        future_cards[card_rot];
                }
                current_card_wins = 0;
                res += future_cards[card_rot] as isize;
                future_cards[card_rot] = 1;
                card_rot = (card_rot + 1) % (MAX_MATCHING_NUMBERS + 1);
                winning_numbers = 0;
                scratching_phase = false;
                idx += CARD_ID_SIZE;
                continue;
            }
            c => panic!("unreachable: {:?}", std::str::from_utf8(&[*c])),
        }
        idx += 1;
    }
    res + future_cards[card_rot] as isize
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
            30
        )
    }
}
