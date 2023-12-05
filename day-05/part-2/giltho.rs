use std::collections::VecDeque;
use std::env::args;
use std::time::Instant;

#[cfg(not(test))]
const SEED_PAIR_AMOUNT: usize = 10;

#[cfg(test)]
const SEED_PAIR_AMOUNT: usize = 2;

#[inline(always)]
fn parse_int(bs: &[u8]) -> usize {
    let mut n = 0;
    for &b in bs {
        n = n * 10 + (b - b'0') as usize;
    }
    n
}

#[inline(always)]
fn parse_triple(line: &[u8]) -> (usize, usize, usize) {
    let mut iter = line.split(|&b| b == b' ');
    (
        parse_int(iter.next().unwrap()),
        parse_int(iter.next().unwrap()),
        parse_int(iter.next().unwrap()),
    )
}

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> usize {
    let mut input = input.as_bytes().split(|&b| b == b'\n');
    let mut seeds = VecDeque::with_capacity(SEED_PAIR_AMOUNT * 30);
    let mut updated_seeds: VecDeque<(usize, usize)> =
        VecDeque::with_capacity(SEED_PAIR_AMOUNT * 30);
    let line = input.next().unwrap();
    let mut words = line.split(|&b| b == b' ').skip(1);
    while let Some(i) = words.next() {
        let first = parse_int(i);
        seeds.push_back((first, parse_int(words.next().unwrap())));
    }
    input.next();
    input.next();
    for line in input {
        match line.first() {
            None => continue,
            Some(b'A'..) => {
                updated_seeds.append(&mut seeds);
                std::mem::swap(&mut seeds, &mut updated_seeds);
                continue;
            }
            _ => (),
        }
        if seeds.is_empty() {
            continue;
        }
        let (dest_start, source_start, length) = parse_triple(line);
        let mut stop_at = seeds.len();
        let mut i = 0;
        while i < stop_at {
            let (seed, seedl) = seeds[i];
            if source_start + length <= seed || seed + seedl <= source_start {
                i += 1;
            } else if source_start <= seed && source_start + length >= seed + seedl {
                seeds.swap_remove_back(i).unwrap();
                updated_seeds.push_back((seed - source_start + dest_start, seedl));
                stop_at -= 1;
            } else if seed <= source_start && seed + seedl >= source_start + length {
                seeds.swap_remove_back(i).unwrap();
                stop_at -= 1;
                let left = (seed, source_start - seed);
                let middle = (dest_start, length); // Already offset
                let right = (source_start + length, seed + seedl - source_start - length);
                if left.1 > 0 {
                    seeds.push_front(left);
                    i += 1;
                    stop_at += 1;
                }
                if right.1 > 0 {
                    seeds.push_front(right);
                    i += 1;
                    stop_at += 1;
                }
                updated_seeds.push_back(middle);
            } else if source_start < seed {
                let left = (
                    seed - source_start + dest_start,
                    source_start + length - seed,
                ); // Already offset
                let right = (source_start + length, seed + seedl - source_start - length);
                seeds.swap_remove_back(i).unwrap();
                seeds.push_front(right);
                i += 1;
                updated_seeds.push_back(left);
            } else {
                let left = (seed, source_start - seed);
                let right = (dest_start, seed + seedl - source_start); // already_offset;
                seeds.swap_remove_back(i).unwrap();
                seeds.push_front(left);
                i += 1;
                updated_seeds.push_back(right);
            }
        }
    }
    std::cmp::min(
        updated_seeds
            .into_iter()
            .fold(usize::MAX, |acc, (i, _)| std::cmp::min(acc, i)),
        seeds
            .into_iter()
            .fold(usize::MAX, |acc, (i, _)| std::cmp::min(acc, i)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run(r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"),
            46
        )
    }
}
