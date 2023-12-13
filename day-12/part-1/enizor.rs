use aoc::enizor::day12::count_arrangements;
use core::panic;
use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> usize {
    let mut row_start = 0;
    let mut cur = 0;
    let bytes = input.as_bytes();
    let mut res = 0;
    while cur < bytes.len() {
        while bytes[cur] != b' ' {
            cur += 1;
        }
        let row = &bytes[row_start..cur];
        cur += 1;
        let mut groups = vec![0];
        while cur < bytes.len() {
            match bytes[cur] {
                b'0'..=b'9' => {
                    let g = groups.last_mut().unwrap();
                    *g *= 10;
                    *g += (bytes[cur] - b'0') as usize;
                }
                b',' => groups.push(0),
                b'\n' => break,
                _ => panic!(
                    "Unexpected input! {} ;  {} ",
                    cur,
                    String::from_utf8(bytes[row_start..=cur].to_owned()).unwrap()
                ),
            };
            cur += 1;
        }
        cur += 1;
        row_start = cur;
        res += count_arrangements(row, &mut groups);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"),
            21
        );
    }
}
