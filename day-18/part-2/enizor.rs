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
    let mut cur = 3;
    let bytes = input.as_bytes();
    let mut area = 0;
    let mut h = 0;
    // see part1 for correction details
    let mut correction = 1;
    while cur < bytes.len() {
        loop {
            match bytes[cur] {
                b' ' => break,
                _ => cur += 1,
            }
        }
        cur += 3;
        let mut parsed_nb = 0;
        for _ in 0..5 {
            match bytes[cur] {
                b'0'..=b'9' => {
                    parsed_nb *= 16;
                    parsed_nb += (bytes[cur] - b'0') as isize;
                }
                b'a'..=b'f' => {
                    parsed_nb *= 16;
                    parsed_nb += (bytes[cur] - b'a' + 10) as isize;
                }
                b'A'..=b'F' => {
                    parsed_nb *= 16;
                    parsed_nb += (bytes[cur] - b'A' + 10) as isize;
                }
                _ => panic!("Unexpected char {:?} at pos {}", bytes[cur] as char, cur),
            }
            cur += 1;
        }
        let (horizontal, sign) = match bytes[cur] {
            b'2' => (true, false),
            b'0' => (true, true),
            b'3' => (false, true),
            b'1' => (false, false),
            _ => panic!("Unexpected char {} for direction", bytes[cur]),
        };
        if horizontal {
            if sign {
                correction += parsed_nb;
                area += h * (parsed_nb);
            } else {
                area -= h * (parsed_nb);
            }
        } else if sign {
            correction += parsed_nb;
            h += parsed_nb;
        } else {
            h -= parsed_nb;
        }
        cur += 6;
    }
    area.abs() + correction
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"),
            952408144115
        )
    }
}
