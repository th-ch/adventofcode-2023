use std::env::args;
use std::time::Instant;

#[cfg(not(test))]
const HANDS: usize = 1000;

#[cfg(test)]
const HANDS: usize = 5;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

#[inline(always)]
fn count_1_bits(x: u16) -> u16 {
    let mut x = (x & 0x5555) + ((x >> 1) & 0x5555);
    x = (x & 0x3333) + ((x >> 2) & 0x3333);
    x = (x & 0x0F0F) + ((x >> 4) & 0x0F0F);
    x = x + (x >> 8);
    x & 31
}

#[inline(always)]
fn value(c: u8) -> u32 {
    match c {
        b'2'..=b'9' => (c - b'2') as u32,
        b'T' => 8,
        b'J' => 9,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        _ => unreachable!(),
    }
}

#[inline(always)]
fn score(hand: &[u8]) -> u32 {
    let mut values: [u32; 5] = unsafe {
        [
            value(*hand.get_unchecked(0)),
            value(*hand.get_unchecked(1)),
            value(*hand.get_unchecked(2)),
            value(*hand.get_unchecked(3)),
            value(*hand.get_unchecked(4)),
        ]
    };
    let raw_score: u32 = (values[0] << (4 * 4))
        + (values[1] << (4 * 3))
        + (values[2] << (4 * 2))
        + (values[3] << 4)
        + values[4];
    values.sort_unstable();
    let mut acc = 0u16;
    let mut cur = 0u16;
    let mut prev = 100;
    let mut set = 0;
    for v in values {
        set |= 1 << v;
        if v == prev {
            cur += 1;
        } else {
            acc = acc * 10 + cur;
            cur = 1;
            prev = v;
        }
    }
    acc = acc * 10 + cur;
    let hand_val = match (count_1_bits(set), acc) {
        (1, _) => 6 << (4 * 5),
        (2, 41 | 14) => 5 << (4 * 5),
        (2, 32 | 23) => 4 << (4 * 5),
        (3, 311 | 131 | 113) => 3 << (4 * 5),
        (3, _) => 2 << (4 * 5),
        (4, _) => 1 << (4 * 5),
        _ => 0,
    };
    hand_val + raw_score
}

fn parse_int(s: &[u8]) -> usize {
    let mut acc = 0;
    for &c in s {
        acc = acc * 10 + (c - b'0') as usize;
    }
    acc
}

fn run(input: &str) -> usize {
    // Your code goes here
    let mut hands = [(0u32, 0usize); HANDS];
    for (line, p) in input
        .as_bytes()
        .split(|&b| b == b'\n')
        .zip(hands.iter_mut())
    {
        let score = score(&line[..5]);
        let bid = parse_int(&line[6..]);
        *p = (score, bid)
    }
    hands.sort_by_key(|k| k.0);
    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + (i + 1) * bid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run(r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"),
            6440
        )
    }
}
