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
    let machines = MachineConfiguration::parse(input);
    machines
        .is_conjunction_of_counters()
        .expect("Input is not a combination of counters. Aborting.")
}
