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
    let mut network = Network::new();
    network.parse_input(input.as_bytes());
    network.run()
}

#[derive(Default)]
struct Network {
    id2name: Vec<Name>,
    paths: Vec<(Id, Id)>,
    instructions: Vec<bool>,
}

type Id = usize;
type Name = usize;

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

    fn new_id(&mut self, name: Name) -> Id {
        self.id2name.push(name);
        self.id2name.len() as Id - 1
    }
    fn get_known_id(&self, name: Name) -> Option<Id> {
        self.id2name
            .iter()
            .position(|s| *s == name)
            .map(|x| x as Id)
    }
    fn get_id(&mut self, name: Name) -> Id {
        self.get_known_id(name).unwrap_or_else(|| self.new_id(name))
    }
    fn new() -> Self {
        Self {
            id2name: Vec::with_capacity(1024),
            paths: vec![(0, 0); 1024],
            instructions: Vec::with_capacity(1024),
        }
    }
    fn get_ids_from_rule(&mut self, rule: &[u8]) -> (Id, Id, Id) {
        let start = self.get_id(Self::parse_name(rule[0], rule[1], rule[2]));
        let left = self.get_id(Self::parse_name(rule[7], rule[8], rule[9]));
        let right = self.get_id(Self::parse_name(rule[12], rule[13], rule[14]));
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
            let (start, left, right) = self.get_ids_from_rule(&input[cur..cur + 15]);
            self.paths[start] = (left, right);
            cur += 17;
        }
    }
    fn run(&mut self) -> usize {
        let mut pos = self.get_id(Self::START);
        let goal = self.get_id(Self::GOAL);
        let mut c = 0;
        loop {
            let (l, r) = self.paths[pos];
            pos = if self.instructions[c % self.instructions.len()] {
                r
            } else {
                l
            };
            c += 1;
            if pos == goal {
                return c;
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
