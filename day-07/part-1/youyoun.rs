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

fn parse_card(c: char) -> u32 {
    match c {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => c.to_digit(10).unwrap() as u32,
    }
}

fn count_chars(str_: &str) -> HashMap<u32, u32> {
    let mut map: HashMap<u32, u32> = HashMap::new();
    for c in str_.chars() {
        let c_int: u32 = parse_card(c);
        let count: &mut u32 = map.entry(c_int).or_insert(0);
        *count += 1;
    }
    map
}

fn eval_hand(str_: &str) -> isize {
    let counter: HashMap<u32, u32> = count_chars(str_);
    if counter.len() == 1 {
        return 6;
    } else if counter.len() == 2 {
        if counter.values().any(|&x: &u32| x == 4) {
            return 5;
        } else {
            return 4;
        }
    } else if counter.len() == 3 {
        if counter.values().any(|&x: &u32| x == 3) {
            return 3;
        } else {
            return 2;
        }
    } else if counter.len() == 4 {
        return 1;
    } else {
        return 0;
    }
}

fn hand_card_values(str_: &str) -> Vec<isize> {
    return str_.chars().map(|c: char| parse_card(c) as isize).collect();
}

fn run(input: &str) -> isize {
    // Your code goes here
    let mut hands: Vec<(&str, isize)> = Vec::new();
    for line in input.lines() {
        let hand: Vec<&str> = line.split_ascii_whitespace().collect::<Vec<&str>>();
        hands.push((hand[0], hand[1].parse::<isize>().unwrap()));
    }
    hands.sort_by_key(|hand_bid: &(&str, isize)| {
        (eval_hand(hand_bid.0), hand_card_values(hand_bid.0))
    });
    hands
        .iter()
        .enumerate()
        .fold(0, |acc: isize, (i, hand_bid)| {
            acc + hand_bid.1 * (i as isize + 1)
        })
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

    #[test]
    fn run_test_equal_till_the_end() {
        assert_eq!(
            run("33322 10
333AA 1
5555A 100
A5555 1000"),
            10 + 2 * 1 + 3 * 100 + 4 * 1000
        )
    }

    #[test]
    fn run_test_rankings() {
        assert_eq!(
            run("AAAAA 1
AAAA2 10
AAA22 100
AAA23 1000
AA223 10000
AA234 100000"),
            100000 + 2 * 10000 + 3 * 1000 + 4 * 100 + 5 * 10 + 6 * 1
        )
    }
}
