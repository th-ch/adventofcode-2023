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
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<isize>>()
        })
        .map(predict_previous_value)
        .sum()
}

fn predict_previous_value(seq: Vec<isize>) -> isize {
    if seq.iter().all(|&x| x == 0) {
        return 0;
    } else if seq.is_empty() {
        panic!("differences never all get to zero")
    };
    let differences = (0..(seq.len() - 1))
        .map(|i| seq[i + 1] - seq[i])
        .collect::<Vec<isize>>();
    seq.first().unwrap() - predict_previous_value(differences)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(predict_previous_value(vec![0, 3, 6, 9, 12, 15]), -3);
        assert_eq!(predict_previous_value(vec![1, 3, 6, 10, 15, 21]), 0);
        assert_eq!(predict_previous_value(vec![10, 13, 16, 21, 30, 45]), 5);
        assert_eq!(
            run("0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"),
            2
        )
    }
}
