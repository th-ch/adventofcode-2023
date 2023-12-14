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
    let mut res = 0;
    let bytes = input.as_bytes();
    let mut x = 0;
    let width = bytes
        .iter()
        .position(|b| *b == b'\n')
        .expect("No newiline in input!");
    let height = (bytes.len() + 1) / (width + 1);
    let mut space_up = vec![height; width];
    let mut h = height;
    for &b in bytes {
        if b == b'\n' {
            x = 0;
            h -= 1;
            continue;
        }
        match b {
            b'.' => {}
            b'#' => {
                space_up[x] = h - 1;
            }
            b'O' => {
                res += space_up[x];
                space_up[x] -= 1;
            }
            _ => panic!("Unexpected char {}", b as char),
        }
        x += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."),
            136
        )
    }
}
