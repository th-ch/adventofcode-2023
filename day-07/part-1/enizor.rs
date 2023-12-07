use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> u64 {
    let mut hands = Vec::with_capacity(1010);
    let mut cur = 0;
    let bytes = input.as_bytes();
    while cur < bytes.len() {
        let h_v = hand_value(&bytes[cur..cur + 5]) as u64;
        cur += 6;
        let mut bet = 0;
        while cur < bytes.len() && bytes[cur].is_ascii_digit() {
            bet *= 10;
            bet += (bytes[cur] - b'0') as u32;
            cur += 1;
        }
        cur += 1;
        hands.push((h_v << 32) | (bet as u64));
    }
    hands.sort_unstable();
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &h)| acc + (h & 0xFFFFFFFFu64) * (i as u64 + 1))
}

const fn card_index(card: u8) -> u8 {
    match card {
        b'2'..=b'9' => card - b'2',
        b'T' => 8,
        b'J' => 9,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        _ => 0,
    }
}

const fn compute_lut() -> [u8; 256] {
    let mut res = [0; 256];
    let mut i = 0;
    while i < 256 {
        res[i] = card_index(i as u8);
        i += 1;
    }
    res
}

const CARD_INDEX: [u8; 256] = compute_lut();

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
        let i = CARD_INDEX[c as usize];
        card_value <<= 4;
        card_value += i as u32;
        combo_count += 1 << (4 * i)
    }
    for _ in 0..13 {
        let count = combo_count & 0xF;
        card_value += STRENGTH_OFFSET << (2 * count);
        // filter out to avoid overflowing the 1s & 0s
        card_value &= COUNT_FILTER;
        combo_count >>= 4;
    }
    card_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        assert_eq!(hand_value(b"32T3K"), 0x101081B);
        assert_eq!(hand_value(b"T55J5"), 0x4083393);
        assert_eq!(hand_value(b"KK677"), 0x20BB455);
        assert_eq!(hand_value(b"KTJJT"), 0x20B8998);
        assert_eq!(hand_value(b"QQQJA"), 0x40AAA9C);
    }

    #[test]
    fn run_test() {
        assert_eq!(
            run("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"),
            6440
        )
    }
}
