use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> isize {
    // Keeps the seeds and their successive conversions
    // Hypothesis: all values are positive
    // => A value > 0 is yet to be converted
    let mut conversions = Vec::<isize>::with_capacity(30);
    let mut words = input.split_ascii_whitespace().skip(1);
    // parse seeds
    for w in &mut words {
        if let Ok(seed) = w.parse() {
            conversions.push(seed);
        } else {
            break;
        }
    }
    let mut count = 0;
    let mut map = [0; 3];
    for w in words {
        if let Ok(val) = w.parse() {
            map[count] = val;
            count += 1;
            if count == 3 {
                for v in conversions.iter_mut() {
                    if (map[1]..(map[1] + map[2])).contains(v) {
                        *v = -(map[0] + (*v - map[1]));
                    }
                }
                count = 0;
            }
        } else {
            for v in conversions.iter_mut() {
                *v = v.abs();
            }
        }
    }
    conversions.iter().map(|v| v.abs()).min().unwrap_or(0)
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
