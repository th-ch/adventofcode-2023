use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn map_id(x: &[u8; 3]) -> usize {
    (x[0] - b'A') as usize * 26 * 26 + (x[1] - b'A') as usize * 26 + (x[2] - b'A') as usize
}

fn run(input: &str) -> isize {
    let mut map = [(0usize, 0usize); 26 * 26 * 26];
    let mut lines = input.as_bytes().split(|&c| c == b'\n');
    let instr = lines.next().unwrap();
    lines.next();
    for line in lines {
        unsafe {
            let ent = map_id(&*(line.as_ptr() as *const [u8; 3]));
            let left = map_id(&*(line.as_ptr().add(7) as *const [u8; 3]));
            let right = map_id(&*(line.as_ptr().add(12) as *const [u8; 3]));
            *map.get_unchecked_mut(ent) = (left, right);
        }
    }
    let mut pos = 0;
    let mut steps = 0;
    'outer: loop {
        for &c in instr {
            if pos == 26 * 26 * 26 - 1 {
                break 'outer;
            }
            match c {
                b'L' => pos = map[pos].0,
                _ => pos = map[pos].1,
            }
            steps += 1;
        }
    }
    // Your code goes here
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run(r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"),
            2
        )
    }
}
