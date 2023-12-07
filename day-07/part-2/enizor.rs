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
    let mut hands = Vec::with_capacity(1010);
    for l in input.lines() {
        hands.push((
            hand_value(&l.as_bytes()[0..5]),
            l[6..].parse::<usize>().unwrap(),
        ));
    }
    hands.sort_unstable();
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &(_h, b))| acc + b * (i + 1))
}

fn card_index(card: u8) -> u8 {
    match card {
        b'2'..=b'9' => card - b'1',
        b'T' => 9,
        b'J' => 0,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        _ => panic!("Unknown card"),
    }
}
// Hand representation:
// same card counts - with 1s and 0s for easier loop -> 12bits
// 0b554433221100

// to filter out 1s and 0s
const COUNT_FILTER: u32 = 0xFF0FFFFF;

// cards = 0b<c0><c1><c2><c3><c4>, each card first on 4 bits -> 20 bits
// e.g. 258JA => cards = 0x0369B
const STRENGTH_OFFSET: u32 = 1 << 20;

fn hand_value(hand: &[u8]) -> u32 {
    let mut card_value = 0;
    // count each card on 4 bits
    let mut combo_count = 0u64;
    for &c in &hand[0..5] {
        let i = card_index(c);
        card_value <<= 4;
        card_value += i as u32;
        combo_count += 1 << (4 * i)
    }
    let nb_jokers = combo_count & 0xF;
    combo_count >>= 4;
    let mut max_combo = 0;
    for _ in 0..12 {
        let count = combo_count & 0xF;
        max_combo = max_combo.max(count);
        card_value += STRENGTH_OFFSET << (2 * count);
        // filter out to avoid overflowing the 1s & 0s
        card_value &= COUNT_FILTER;
        combo_count >>= 4;
    }
    // increase max combo by number of jokers:
    // reduce by one the max combo if >= 2
    card_value -= COUNT_FILTER & (STRENGTH_OFFSET << (2 * max_combo));
    // add jokers to max combo
    card_value += STRENGTH_OFFSET << (2 * (max_combo + nb_jokers));
    card_value &= COUNT_FILTER;
    card_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        assert_eq!(hand_value(b"32T3K"), 0x102192B);
        assert_eq!(hand_value(b"T55J5"), 0x10094404);
        assert_eq!(hand_value(b"KK677"), 0x20BB566);
        assert_eq!(hand_value(b"KTJJT"), 0x100B9009);
        assert_eq!(hand_value(b"QQQJA"), 0x100AAA0C);
    }

    #[test]
    fn run_test() {
        assert_eq!(
            run("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"),
            5905
        )
    }
}
