use std::collections::HashMap;
use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn split_by_delimiter(input: &str, delimiter: char) -> Vec<&str> {
    return input
        .split(delimiter)
        .map(|s| s.trim_matches(delimiter).trim())
        .collect();
}

fn run(input: &str) -> isize {
    let mut sum_: isize = 0;

    let color_limits: HashMap<&str, isize> = [("red", 12), ("green", 13), ("blue", 14)]
        .iter()
        .cloned()
        .collect();

    for game in input.lines() {
        let parsed_line: Vec<&str> = split_by_delimiter(game.trim(), ':');
        let game_id = parsed_line[0]
            .trim_start_matches("Game ")
            .parse::<isize>()
            .unwrap();

        let mut is_possible: bool = true;
        for set in split_by_delimiter(parsed_line[1], ';') {
            for ball in split_by_delimiter(set.trim(), ',') {
                let parsed_record = split_by_delimiter(ball, ' ');
                let count = parsed_record[0];
                let color = parsed_record[1];
                if count.parse::<isize>().unwrap() > *color_limits.get(color.trim()).unwrap() {
                    is_possible = false;
                    break;
                }
            }
            if !is_possible {
                break;
            }
        }
        if is_possible {
            sum_ += game_id;
        }
    }
    sum_
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            8
        )
    }
}
