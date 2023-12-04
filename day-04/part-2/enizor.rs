use aoc::enizor::bitset::*;
use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

// Suppose that scratched numbers use 2 digits at most
const BITSET_SIZE: usize = bitset_size(99);
type WinningNumbers = ArrayBitSet<BITSET_SIZE>;

// at most 25 match per card on my input
// => use 32 to have some margin and easier modulos
const MAX_CARDS: usize = 32;

fn run(input: &str) -> usize {
    let mut bytes = b"\n".iter().chain(input.as_bytes()).chain(b"\n".iter());
    let mut winning_set = WinningNumbers::default();
    let mut parsed_num = 0;
    let mut parsing_winning = true;
    let mut win_count = 0;
    let mut cards_count = [0; MAX_CARDS];
    let mut card_nb = 0;
    let mut res: usize = 0;
    while let Some(&b) = bytes.next() {
        if b.is_ascii_digit() {
            parsed_num *= 10;
            parsed_num += (b - b'0') as usize;
        } else {
            if parsed_num != 0 {
                if parsing_winning {
                    winning_set.set(parsed_num);
                } else if winning_set.test(parsed_num) {
                    win_count += 1;
                }
                parsed_num = 0;
            }
            match b {
                b'|' => {
                    parsing_winning = false;
                }
                b'\n' => {
                    let c = cards_count[card_nb];
                    for i in 1..=win_count {
                        cards_count[(card_nb + i) % MAX_CARDS] += c;
                    }
                    res += c;
                    cards_count[card_nb] = 0;
                    winning_set = Default::default();
                    win_count = 0;
                    parsing_winning = true;
                    for &b in bytes.by_ref() {
                        if b == b':' {
                            card_nb = (card_nb + 1) % MAX_CARDS;
                            cards_count[card_nb] += 1;
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    res + cards_count.iter().sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("Card 1: 41 48 83 86 17 | 83 86 1 6 31 17 1 9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
            30
        )
    }
}
