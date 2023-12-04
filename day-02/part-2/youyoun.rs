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

    for game in input.lines() {
        let parsed_line: Vec<&str> = split_by_delimiter(game.trim(), ':');

        let mut max_colors: HashMap<&str, isize> = [("red", 0), ("green", 0), ("blue", 0)]
            .iter()
            .cloned()
            .collect();
        for set in split_by_delimiter(parsed_line[1], ';') {
            for ball in split_by_delimiter(set.trim(), ',') {
                let parsed_record = split_by_delimiter(ball, ' ');
                let count: &str = parsed_record[0];
                let color: &str = parsed_record[1];

                if max_colors.get(color.trim()).unwrap() < &count.parse::<isize>().unwrap() {
                    max_colors.insert(color.trim(), count.parse::<isize>().unwrap());
                }
            }
        }
        sum_ += max_colors.values().product::<isize>();
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
            2286
        )
    }
}
