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
    starts: Vec<Id>,
}

type Id = u32;
type Name = u32;

fn binary_gcd(mut u: usize, mut v: usize) -> usize {
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }
    // maximum power of 2 dividing both u & v
    let max_power_2 = (u | v).trailing_zeros();

    // Turn both to their odd parts
    u >>= u.trailing_zeros();
    v >>= v.trailing_zeros();

    // Use the simple gcd(u, v) = gcd(|u − v|, min(u, v)), for the odd parts of u and v.
    // as u-v is even but min(u, v) is odd, we can continue to strip it off the even part
    while u != v {
        if u < v {
            core::mem::swap(&mut u, &mut v);
        }
        u -= v;
        u >>= u.trailing_zeros();
    }

    // Multiply back the even part
    u << max_power_2
}

fn lcm(u: usize, v: usize) -> usize {
    u * v / binary_gcd(u, v)
}

impl Network {
    const fn parse_name(a: u8, b: u8, c: u8) -> Name {
        let mut res = (b'Z' - a) as Name;
        res <<= 5;
        res |= (b'Z' - b) as Name;
        res <<= 5;
        res |= (b'Z' - c) as Name;
        res
    }
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
            starts: Vec::with_capacity(8),
        }
    }
    fn get_ids_from_rule(&mut self, rule: &[u8]) -> (Id, Id, Id) {
        let start = self.get_id(Self::parse_name(rule[0], rule[1], rule[2]));
        if rule[2] == b'A' {
            self.starts.push(start);
        }
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
            self.paths[start as usize] = (left, right);
            cur += 17;
        }
    }
    fn run(&mut self) -> usize {
        let mut c = 0;
        let mut cycles = vec![0; self.starts.len()];
        let mut stopped = 0;
        loop {
            let right = self.instructions[c % self.instructions.len()];
            c += 1;
            for (i, pos) in self.starts.iter_mut().enumerate() {
                if cycles[i] == 0 {
                    let (l, r) = self.paths[*pos as usize];
                    *pos = if right { r } else { l };
                    if (self.id2name[*pos as usize] & 0x1F) == 0 {
                        stopped += 1;
                        cycles[i] = c;
                    }
                }
            }
            if stopped == self.starts.len() {
                return cycles.iter().fold(1, |acc, l| lcm(acc, *l));
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
        assert_eq!(
            run("LR

UUA = (UUB, XXX)
UUB = (XXX, UUZ)
UUZ = (UUB, XXX)
VVA = (VVB, XXX)
VVB = (VVC, VVC)
VVC = (VVZ, VVZ)
VVZ = (VVB, VVB)
XXX = (XXX, XXX)"),
            6
        );
    }
}
