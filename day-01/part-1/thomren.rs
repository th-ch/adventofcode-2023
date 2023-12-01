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
    input.lines().map(get_calibration_value).sum()
}

fn get_calibration_value(line: &str) -> usize {
    let values = line
        .chars()
        .filter(|&c| c >= '0' && c <= '9')
        .collect::<Vec<char>>();
    if values.len() == 0 {
        return 0;
    }
    return (*values.first().unwrap() as usize - '0' as usize) * 10
        + (*values.last().unwrap() as usize - '0' as usize);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"),
            142
        )
    }
}
