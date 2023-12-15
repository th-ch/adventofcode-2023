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
        'J' => 1,
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
    let mut counter: HashMap<u32, u32> = count_chars(str_);
    if counter.keys().any(|x| x == &1) {
        if counter[&1] == 5 {
            return 6;
        } else {
            let jokers: u32 = counter[&1];
            counter.remove(&1);
            for (_, v) in counter.iter_mut() {
                *v += jokers;
            }
        }
    }
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
        assert_eq!(run("Test example"), 0)
    }
}
