use std::collections::HashMap;
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
    let (hand_strengths, bids) = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(hand, bid)| {
            (
                hand_strength(hand.as_bytes().try_into().unwrap()),
                bid.parse::<usize>().unwrap(),
            )
        })
        .unzip::<usize, usize, Vec<usize>, Vec<usize>>();
    let mut sorted_indices = (0..hand_strengths.len()).collect::<Vec<_>>();
    sorted_indices.sort_by_key(|&i| hand_strengths[i]);
    sorted_indices
        .iter()
        .enumerate()
        .map(|(rank, &index)| (rank + 1) * bids[index])
        .sum()
}

fn hand_strength(hand: [u8; 5]) -> usize {
    let hand = hand.map(|c| match c {
        b'2'..=b'9' => c - b'0',
        b'T' => 10,
        b'J' => 11,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        _ => unreachable!(),
    });
    let mut counts = HashMap::new();
    for c in hand.iter() {
        *counts.entry(c).or_insert(0) += 1;
    }
    let mut counts_values = counts.into_values().collect::<Vec<_>>();
    counts_values.sort_by(|a, b| b.cmp(a));
    let m = counts_values[0];
    let s = *counts_values.get(1).unwrap_or(&0);
    let type_score = match (m, s) {
        (5, _) => 8,
        (4, _) => 7,
        (3, 2) => 6,
        (3, _) => 3,
        (2, 2) => 2,
        (2, _) => 1,
        (1, _) => 0,
        _ => unreachable!(),
    } as usize;
    (type_score << 20)
        + hand
            .iter()
            .enumerate()
            .map(|(i, &c)| (c as usize) << (4 * (4 - i)))
            .sum::<usize>()
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
QQQJA 483"),
            6440
        )
    }
}
