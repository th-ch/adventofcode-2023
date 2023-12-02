use aoc::enizor::utils::*;
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
        .map(test_game)
        .reduce(|acc, v| acc + v)
        .unwrap_or(0)
}

fn test_game(game: &[u8]) -> usize {
    let mut words = game.split(|b| *b == b' ').skip(1);
    let id_w = words.next().expect("No game ID");
    let id = consume_numeral(id_w);
    while let Some(number_w) = words.next() {
        let number = consume_numeral(number_w);
        let color = words.next().expect("No number of cubes");
        let max = match color[0] {
            b'r' => 12,
            b'g' => 13,
            b'b' => 14,
            _ => panic!(
                "Unexpected color {} !",
                String::from_utf8(color.into()).unwrap()
            ),
        };
        if number > max {
            return 0;
        }
    }
    id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            test_game(b"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            1
        );
        assert_eq!(
            test_game(b"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            2
        );
        assert_eq!(
            test_game(b"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            0
        );
        assert_eq!(
            test_game(b"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            0
        );
        assert_eq!(
            test_game(b"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            5
        )
    }
}
