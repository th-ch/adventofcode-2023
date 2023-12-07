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

fn char_to_rank() -> [u8; 256] {
    let mut ranks = [0; 256];
    for (i, card) in [
        b'J', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'Q', b'K', b'A',
    ]
    .into_iter()
    .enumerate()
    {
        ranks[card as usize] = (i + 1) as u8;
    }
    ranks
}

#[repr(u8)]
#[derive(Debug)]
enum Hand {
    Five = 6,
    Four = 5,
    Full = 4,
    Three = 3,
    TwoPairs = 2,
    Pair = 1,
    HighCard = 0,
}

fn card_counts_to_hand(hand: &[u8; 5]) -> Hand {
    use Hand::*;
    let mut card_counts = [0; 14];
    for &card in hand {
        card_counts[card as usize] += 1 as u8;
    }
    let max_idx = card_counts[2..]
        .iter()
        .enumerate()
        .map(|(i, &c)| (c, i + 2))
        .max()
        .unwrap()
        .1;
    card_counts[max_idx] += card_counts[1];

    let mut pairs = 0;
    let mut three = false;
    for i in 2..card_counts.len() {
        match card_counts[i] {
            5 => return Five,
            4 => return Four,
            3 => three = true,
            2 => pairs += 1,
            _ => {}
        }
    }
    if three && pairs == 1 {
        Full
    } else if three {
        Three
    } else if pairs == 2 {
        TwoPairs
    } else if pairs == 1 {
        Pair
    } else {
        HighCard
    }
}

fn run(input: &str) -> isize {
    let ranks = char_to_rank();
    let mut t = Tokenizer::new(input);
    let mut hands = Vec::with_capacity(16);
    while t.curr_char().is_some() {
        let hand_str = t.consume_while(|b| b.is_ascii_alphanumeric());
        t.advance(1);
        let bid = t.consume_u32().unwrap();
        t.advance(1);

        let mut hand = [0; 5];
        for (i, &c) in hand_str.as_bytes().iter().enumerate() {
            hand[i] = ranks[c as usize];
        }

        let hand_type = card_counts_to_hand(&hand);
        hands.push((hand_type as u8, hand, bid))
    }
    hands.sort();

    let mut total = 0;
    for (i, (_, _, bid)) in hands.iter().enumerate() {
        total += ((i + 1) * (*bid) as usize) as isize
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"),
            5905
        );
    }

    #[test]
    fn run_custom_test() {
        assert_eq!(
            run("2J3JA 1
A2J3J 2
"),
            5
        );

        assert_eq!(
            run("2J3JA 1
3J3J2 2
JJJ23 1
J2345 1
"),
            5
        );
    }
}
