use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provNamee an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> usize {
    let mut network = Network::new();
    network.parse_input(input.as_bytes());
    network.run()
}

type Name = u32;

const MAX_LENGTH: usize = 32 * 32 * 32;
struct Network {
    paths: [(Name, Name); MAX_LENGTH],
    instructions: Vec<bool>,
}

impl Network {
    const fn parse_name(a: u8, b: u8, c: u8) -> Name {
        let mut res = (a - b'A') as Name;
        res <<= 5;
        res |= (b - b'A') as Name;
        res <<= 5;
        res |= (c - b'A') as Name;
        res
    }
    const START: Name = Self::parse_name(b'A', b'A', b'A');
    const GOAL: Name = Self::parse_name(b'Z', b'Z', b'Z');

    fn new() -> Self {
        Self {
            paths: [(0, 0); MAX_LENGTH],
            instructions: Vec::with_capacity(1024),
        }
    }
    fn get_names_from_rule(&mut self, rule: &[u8]) -> (Name, Name, Name) {
        let start = Self::parse_name(rule[0], rule[1], rule[2]);
        let left = Self::parse_name(rule[7], rule[8], rule[9]);
        let right = Self::parse_name(rule[12], rule[13], rule[14]);
        (start, left, right)
    }
    fn parse_input(&mut self, input: &[u8]) {
        let mut cur = 0;
        loop {
            match input[cur] {
                b'L' => self.instructions.push(false),
                b'R' => self.instructions.push(true),
                _ => break,
            }
            cur += 1;
        }
        cur += 2;
        while cur + 15 < input.len() {
            let (start, left, right) = self.get_names_from_rule(&input[cur..cur + 15]);
            self.paths[start as usize] = (left, right);
            cur += 17;
        }
    }
    fn run(&mut self) -> usize {
        let mut pos = Self::START;
        let goal = Self::GOAL;
        let mut c = 0;
        loop {
            for b in &self.instructions {
                let (l, r) = self.paths[pos as usize];
                pos = if *b { r } else { l };
                c += 1;
                if pos == goal {
                    return c;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"),
            2
        );
        assert_eq!(
            run("LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"),
            6
        );
    }
}
