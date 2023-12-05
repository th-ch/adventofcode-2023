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
    fn get_ranges(&self, sorted_ranges: &[(usize, usize)], out: &mut Vec<(usize, usize)>) {
        let mut i = 0;
        for &(mut start, end) in sorted_ranges {
            i = loop {
                let Some(((lower_bound, upper_bound), _)) = self.entries.get(i) else {
                    break i;
                };
                if start <= *lower_bound || start < *upper_bound {
                    break i;
                }
                i += 1;
            };
            loop {
                let Some(&((lower_bound, upper_bound), dest)) = self.entries.get(i) else {
                    out.push((start, end));
                    break;
                };
                if start < lower_bound {
                    let end_range = std::cmp::min(end, lower_bound);
                    out.push((start, end_range));
                    start = end_range;
                }
                if start == end {
                    break;
                }
                let end_range = std::cmp::min(end, upper_bound);
                out.push((start - lower_bound + dest, end_range - lower_bound + dest));
                start = end_range;
                if start == end {
                    break;
                }
                i += 1;
            }
        }
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
    Map { entries }
}

fn run(input: &str) -> isize {
    let mut t = Tokenizer::new(input);
    let mut seeds = Vec::new();

    t.consume_fixed("seeds:").unwrap();
    while t.curr_char() != Some(b'\n') {
        t.consume_fixed(" ").unwrap();
        let start = t.consume_u32().unwrap() as usize;
        t.consume_fixed(" ").unwrap();
        let range = t.consume_u32().unwrap() as usize;
        seeds.push((start, start + range));
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
            let mut ranges = vec![seed];
            let mut mapped_ranges = Vec::new();
            for m in &maps {
                ranges.sort();
                m.get_ranges(&ranges, &mut mapped_ranges);
                std::mem::swap(&mut ranges, &mut mapped_ranges);
                mapped_ranges.truncate(0);
            }
            // dbg!(&ranges);
            ranges.into_iter().min().unwrap().0
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
            46
        )
    }
}
