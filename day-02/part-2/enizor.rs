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
    let bytes = input.as_bytes();
    bytes
        .split(|b| *b == b'\n')
        .map(game_power)
        .reduce(|acc, v| acc + v)
        .unwrap_or(0)
}

fn game_power(game: &[u8]) -> usize {
    let mut max_r = 0;
    let mut max_g = 0;
    let mut max_b = 0;
    let mut words = game.split(|b| *b == b' ').skip(2);
    while let Some(number_w) = words.next() {
        let number = consume_numeral(number_w);
        let color = words.next().expect("No number of cubes");
        let max = match color[0] {
            b'r' => &mut max_r,
            b'g' => &mut max_g,
            b'b' => &mut max_b,
            _ => panic!(
                "Unexpected color {} !",
                String::from_utf8(color.into()).unwrap()
            ),
        };
        *max = std::cmp::max(*max, number);
    }
    max_r * max_g * max_b
}

fn consume_numeral(bytes: &[u8]) -> usize {
    let mut res = 0;
    let mut cur = 0;
    while cur < bytes.len() {
        if bytes[cur] >= b'0' && bytes[cur] <= b'9' {
            res *= 10;
            res += (bytes[cur] - b'0') as usize;
            cur += 1;
        } else {
            break;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            game_power(b"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            48
        );
        assert_eq!(
            game_power(b"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            12
        );
        assert_eq!(
            game_power(b"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            1560
        );
        assert_eq!(
            game_power(b"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            630
        );
        assert_eq!(
            game_power(b"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            36
        )
    }
}
