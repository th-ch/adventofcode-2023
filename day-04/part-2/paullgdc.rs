use std::env::args;
use std::time::Instant;

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

    let mut games = 0;
    let mut duplicates: Vec<isize> = Vec::new();

    let mut winning_cards = Vec::new();
    let mut game_number = 0;
    loop {
        if t.curr_char().is_none() {
            break;
        }

        let mut card_won = 0;

        winning_cards.truncate(0);

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
            let available = t.consume_u32().unwrap() as u8;
            for winning_card in &winning_cards {
                if available == *winning_card {
                    card_won += 1;
                }
            }
        }
        if game_number == duplicates.len() {
            duplicates.push(0);
        }
        duplicates[game_number] += 1;

        let multiplier = duplicates[game_number];

        games += multiplier;
        if card_won > 0 {
            for i in 1..=card_won {
                if duplicates.get(game_number + i).is_none() {
                    duplicates.push(0);
                }
                duplicates[game_number + i] += multiplier;
            }
        }
        game_number += 1;
    }
    games
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
            30
        )
    }
}
