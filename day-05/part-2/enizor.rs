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
    // Keeps the seeds and their successive conversions using rages start..end
    // Hypothesis: all values are positive
    // => A value > 0 is yet to be converted
    let mut conversions = Vec::<(isize, isize)>::with_capacity(120);
    let mut words = input.split_ascii_whitespace().skip(1);
    // parse seeds
    let mut count = 0;
    for w in &mut words {
        if let Ok(seed) = w.parse() {
            if count % 2 == 0 {
                conversions.push((seed, 0));
            } else {
                let (start, end) = conversions.last_mut().unwrap();
                *end = *start + seed;
            }
            count += 1;
        } else {
            break;
        }
    }
    count = 0;
    let mut map = [0; 3];
    // When mapping a part of a range, push temporarily the non-converted parts here
    let mut new_ranges = Vec::<(isize, isize)>::with_capacity(15);
    for w in words {
        if let Ok(val) = w.parse() {
            map[count] = val;
            count += 1;
            if count == 3 {
                let m_start = map[1];
                let m_end = map[1] + map[2];
                for (start, end) in conversions.iter_mut() {
                    if *start < 0 || m_end <= *start || m_start >= *end {
                        // no overlap or already converted
                        continue;
                    }
                    if m_start <= *start {
                        *start = -(map[0] + (*start - m_start));
                    } else {
                        new_ranges.push((*start, m_start));
                        *start = -map[0];
                    }
                    if m_end >= *end {
                        *end = -(map[0] + (*end - m_start));
                    } else {
                        new_ranges.push((m_end, *end));
                        *end = -(map[0] + (m_end - m_start));
                    }
                }
                // Push the splitted, non-converted parts
                // don't bother compacting, all the ranges are apart on my input
                for &r in &new_ranges {
                    conversions.push(r);
                }
                new_ranges.clear();
                count = 0;
            }
        } else {
            for (start, end) in conversions.iter_mut() {
                *start = start.abs();
                *end = end.abs();
            }
        }
    }
    conversions.iter().map(|v| v.0.abs()).min().unwrap_or(0)
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
