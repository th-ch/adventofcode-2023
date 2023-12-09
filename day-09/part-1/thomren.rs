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
        .map(predict_next_value)
        .sum()
}

fn predict_next_value(seq: Vec<isize>) -> isize {
    if seq.iter().all(|&x| x == 0) {
        return 0;
    } else if seq.is_empty() {
        panic!("differences never all get to zero")
    };
    let differences = (0..(seq.len() - 1))
        .map(|i| seq[i + 1] - seq[i])
        .collect::<Vec<isize>>();
    seq.last().unwrap() + predict_next_value(differences)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(predict_next_value(vec![0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(predict_next_value(vec![1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(predict_next_value(vec![10, 13, 16, 21, 30, 45]), 68);
        assert_eq!(
            run("0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"),
            114
        )
    }
}
