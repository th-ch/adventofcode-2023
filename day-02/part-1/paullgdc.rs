use std::env::args;
use std::time::Instant;

use aoc::paullgdc::tokenizer::Tokenizer;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn max_color(color: &str) -> u32 {
    match color {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
        _ => 0
    }
}

fn run(input: &str) -> isize {
    let mut t = Tokenizer::new(input);
    let mut tot = 0;
    'games: loop {
        match t.curr_char(){
            None => break,
            Some(b'\n')=> {t.advance(1); continue}
            _ => {}
        }
        t.consume_fixed("Game ");
        let game_id = t.consume_u32().unwrap();
        t.advance(2);
        loop {
            let curr = t.curr_char();
            if curr.is_none() || curr == Some(b'\n') {
                break;
            }
            let cubes = t.consume_u32().unwrap();
            t.advance(1);
            let color = t.consume_name();
            if cubes > max_color(color) {
                t.consume_until(b'\n');
                t.advance(1);
                continue 'games;
            }
            match t.curr_char() {
                Some(b'\n') => {t.advance(1); break;},
                Some(b',' | b';') => t.advance(2),
                None => break,
                _ => panic!()
            }
        }
        tot += game_id;
    }
    tot as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"), 8)
    }
}
