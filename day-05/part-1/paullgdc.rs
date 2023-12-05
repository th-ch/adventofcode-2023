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

struct Map {
    entries: Vec<((usize, usize), usize)>,
}

impl Map {
    fn get(&self, key: usize) -> usize {
        // match self
        //     .entries
        //     .binary_search_by(|((start, _), _)| start.cmp(&key))
        // {
        //     Ok(i) => self.entries[i].1,
        //     Err(i) => {
        //         if i == 0 {
        //             return key;
        //         }
        //         let e = self.entries[i - 1];
        //         if (e.0 .0..e.0 .1).contains(&key) {
        //             e.1 + (key - e.0 .0)
        //         } else {
        //             key
        //         }
        //     }
        // }
        for e in &self.entries {
            if (e.0 .0..e.0 .1).contains(&key) {
                return e.1 + (key - e.0 .0);
            }
        }
        return key;
    }
}

fn parse_map(t: &mut Tokenizer) -> Map {
    let mut entries = Vec::with_capacity(5);
    while t.curr_char().is_some() && t.curr_char() != Some(b'\n') {
        let dest = t.consume_u32().unwrap() as usize;
        t.advance(1);
        let source_start = t.consume_u32().unwrap() as usize;
        t.advance(1);
        let range = t.consume_u32().unwrap() as usize;
        t.advance(1);
        entries.push(((source_start, source_start + range), dest));
    }
    entries.sort();
    // entries.etr
    Map { entries }
}

fn run(input: &str) -> isize {
    let mut t = Tokenizer::new(input);
    let mut seeds = Vec::new();

    t.consume_fixed("seeds:").unwrap();
    while t.curr_char() != Some(b'\n') {
        t.consume_fixed(" ").unwrap();
        seeds.push(t.consume_u32().unwrap() as usize);
    }

    t.consume_fixed("\n\nseed-to-soil map:\n").unwrap();
    let seed_to_soil = parse_map(&mut t);
    t.consume_fixed("\nsoil-to-fertilizer map:\n").unwrap();
    let soil_to_fertilizer = parse_map(&mut t);
    t.consume_fixed("\nfertilizer-to-water map:\n").unwrap();
    let fertilizer_to_water = parse_map(&mut t);
    t.consume_fixed("\nwater-to-light map:\n").unwrap();
    let water_to_light = parse_map(&mut t);
    t.consume_fixed("\nlight-to-temperature map:\n").unwrap();
    let light_to_temperature = parse_map(&mut t);
    t.consume_fixed("\ntemperature-to-humidity map:\n").unwrap();
    let temp_to_humidity = parse_map(&mut t);
    t.consume_fixed("\nhumidity-to-location map:\n").unwrap();
    let humidity_to_loc = parse_map(&mut t);

    let maps = vec![
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temp_to_humidity,
        humidity_to_loc,
    ];

    seeds
        .iter()
        .map(|&seed| {
            let mut key = seed;
            for m in &maps {
                key = m.get(key);
            }
            key
        })
        .min()
        .unwrap() as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("seeds: 79 14 55 13

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
