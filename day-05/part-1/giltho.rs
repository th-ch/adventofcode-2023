use std::env::args;
use std::time::Instant;

#[cfg(not(test))]
const SEED_AMOUNT: usize = 20;

#[cfg(test)]
const SEED_AMOUNT: usize = 4;

const ALL_UPDATED: u32 = (1 << SEED_AMOUNT) - 1;

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
    let mut seeds = [0usize; SEED_AMOUNT];
    let mut seed_updated: u32 = 0;
    let line = input.next().unwrap();
    let words = line.split(|&b| b == b' ').skip(1).enumerate();
    for (i, w) in words {
        seeds[i] = parse_int(w);
    }
    input.next();
    input.next();
    for line in input {
        match line.first() {
            None => continue,
            Some(b'A'..) => {
                seed_updated = 0;
                continue;
            }
            _ => (),
        }
        if seed_updated == ALL_UPDATED {
            continue;
        }
        let (dest_start, source_start, length) = parse_triple(line);
        for (i, seed) in seeds.iter_mut().enumerate() {
            if (seed_updated & (1 << i) == 0)
                && (source_start..(source_start + length)).contains(seed)
            {
                *seed = dest_start + (*seed - source_start);
                seed_updated |= 1 << i;
            }
        }
    }
    seeds.into_iter().fold(usize::MAX, std::cmp::min)
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
            35
        )
    }
}
