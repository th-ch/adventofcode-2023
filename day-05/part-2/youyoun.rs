use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn parse_map_block(block: &str) -> Vec<Vec<usize>> {
    let mut map: Vec<Vec<usize>> = Vec::new();
    for line in block.split("\n").skip(1) {
        map.push(
            line.split(" ")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>(),
        );
    }
    map
}

fn run(input: &str) -> usize {
    // Your code goes here
    let blocks: Vec<&str> = input.split("\n\n").collect();

    let seeds_ranges: Vec<usize> = blocks[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut seeds: Vec<(usize, usize)> = Vec::new();
    for i in 0..seeds_ranges.len() / 2 {
        seeds.push((
            seeds_ranges[2 * i],
            seeds_ranges[2 * i + 1] + seeds_ranges[2 * i] - 1,
        ));
    }

    for block in blocks.iter().skip(1) {
        let parsed_block = parse_map_block(block);
        let mut new_seeds: Vec<(usize, usize)> = Vec::new();
        let mut to_process: Vec<(usize, usize)> = seeds.clone();
        while to_process.len() > 0 {
            let seed_ = to_process.pop().unwrap();
            let mut modified = false;
            for dst_src_range in parsed_block.iter() {
                let src_range = (dst_src_range[1], dst_src_range[1] + dst_src_range[2] - 1);
                let dst_range = (dst_src_range[0], dst_src_range[0] + dst_src_range[2] - 1);

                if src_range.0 > seed_.1 || src_range.1 < seed_.0 {
                    continue;
                } else if seed_.0 >= src_range.0 && seed_.0 <= src_range.1 {
                    if seed_.1 <= src_range.1 {
                        new_seeds.push((
                            dst_range.0 + seed_.0 - src_range.0,
                            dst_range.0 + seed_.1 - src_range.0,
                        ));
                        modified = true;
                        break;
                    } else {
                        new_seeds.push((dst_range.0 + seed_.0 - src_range.0, dst_range.1));
                        to_process.push((src_range.1 + 1, seed_.1));
                        modified = true;
                        break;
                    }
                } else if seed_.1 >= src_range.0 && seed_.1 <= src_range.1 {
                    new_seeds.push((dst_range.0, dst_range.0 + seed_.1 - src_range.0));
                    to_process.push((seed_.0, src_range.0 - 1));
                    modified = true;
                    break;
                }
            }
            if !modified {
                new_seeds.push(seed_);
            }
        }
        seeds = new_seeds;
    }
    seeds.iter().map(|x| x.0).min().unwrap() as usize
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
