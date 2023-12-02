use std::env::args;
use std::time::Instant;

#[inline(always)]
fn len_game_size(i: isize) -> usize {
    match i {
        0..=9 => 1,
        10..=99 => 2,
        _ => 3,
    }
}

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> usize {
    let input = input.as_bytes();
    let mut game: isize = 0;
    let mut res: usize = 0;
    for line in input.split(|&c| c == b'\n') {
        let mut acc: u8 = 0;
        let mut blues: u8 = 0;
        let mut reds: u8 = 0;
        let mut greens: u8 = 0;
        game += 1;
        let mut idx = 7 + len_game_size(game);
        while idx < line.len() {
            match unsafe { line.get_unchecked(idx) } {
                c @ b'0'..=b'9' => {
                    acc = acc * 10 + (c - b'0');
                }
                b'b' => {
                    blues = std::cmp::max(blues, acc);
                    acc = 0;
                    idx += 4;
                }
                b'r' => {
                    reds = std::cmp::max(reds, acc);
                    acc = 0;
                    idx += 3;
                }
                b'g' => {
                    greens = std::cmp::max(greens, acc);
                    acc = 0;
                    idx += 5;
                }
                b' ' => (),
                _ => unreachable!(),
            }
            idx += 1;
        }
        res += (blues as usize) * (reds as usize) * (greens as usize);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run(r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            2286
        )
    }
}
