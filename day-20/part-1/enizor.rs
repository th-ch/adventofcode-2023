use aoc::enizor::day20::MachineConfiguration;
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
    let mut machines = MachineConfiguration::parse(input);
    let mut low_count = 0;
    let mut high_count = 0;
    for _it in 0..1000 {
        let (low, high) = machines.press_button();
        low_count += low;
        high_count += high;
    }
    low_count * high_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"),
            32000000
        );
        assert_eq!(
            run("broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"),
            11687500
        );
    }
}
