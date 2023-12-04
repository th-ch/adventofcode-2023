use std::env::args;
use std::time::Instant;
use std::cmp::Ordering;


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
    let mut total = 0;
    let mut winning_cards = Vec::new();
    let mut available_cards = Vec::new();
    loop {
        if t.curr_char().is_none() {
            break;
        }
        winning_cards.truncate(0);
        available_cards.truncate(0);

        t.consume_until(b':');
        t.advance(1);
        loop {
            t.consume_whitespaces();
            if t.curr_char() == Some(b'|') {
                t.advance(1);
                break;
            }
            winning_cards.push(t.consume_u32().unwrap() as u8);
        }
        loop {
            t.consume_whitespaces();
            if t.curr_char().is_none() || t.curr_char() == Some(b'\n') {
                t.advance(1);
                break;
            }
            available_cards.push(t.consume_u32().unwrap() as u8);
        }

        let mut card_won = 0;
        // winning_cards.sort();
        // available_cards.sort();
        // let mut winning_idx = 0;
        // let mut available_idx = 0;
        // use Ordering::*;
        // while winning_idx < winning_cards.len() && available_idx < available_cards.len() {
        //     match winning_cards[winning_idx].cmp(&available_cards[available_idx]) {
        //         Less => {
        //             winning_idx += 1;
        //         }
        //         Greater => {
        //             available_idx += 1;
        //         }
        //         Equal => {
        //             card_won += 1;
        //             winning_idx += 1;
        //             available_idx += 1;
        //         }
        //     }
        // }
        for winning_card in &winning_cards {
            for available_card in &available_cards {
                if available_card == winning_card {
                    card_won += 1;
                }
            }
        }
        if card_won > 0 {
            total += 1 << (card_won - 1);
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
            run("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"),
            13
        )
    }
}
