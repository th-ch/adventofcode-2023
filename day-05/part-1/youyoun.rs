use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn parse_map_block(block: &str) -> Vec<Vec<isize>> {
    let mut map: Vec<Vec<isize>> = Vec::new();
    for line in block.split("\n").skip(1) {
        map.push(
            line.split(" ")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<isize>>(),
        );
    }
    map
}

fn run(input: &str) -> isize {
    // Your code goes here
    let blocks: Vec<&str> = input.split("\n\n").collect();

    let mut seeds: Vec<isize> = blocks[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect();

    // println!("{:?}", seeds);
    for block in blocks.iter().skip(1) {
        // println!("{:?}", block);
        let parsed_block = parse_map_block(block);
        // println!("{:?}", parsed_block);
        let mut seed_location: Vec<isize> = seeds.clone();
        for (idx, seed) in seeds.iter().enumerate() {
            // println!("{}", seed);
            for dst_src_range in parsed_block.iter() {
                // println!("{:?}", dst_src_range);
                if seed >= &dst_src_range[1] && seed <= &(dst_src_range[1] + dst_src_range[2]) {
                    // println!("{} -> {}", seed, dst_src_range[0] + seed - dst_src_range[1]);
                    seed_location[idx] = dst_src_range[0] + seed - dst_src_range[1];
                    break;
                }
            }
        }
        seeds = seed_location;
        // println!("{:?}", seeds);
        // println!("")
    }
    *seeds.iter().min().unwrap() as isize
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
