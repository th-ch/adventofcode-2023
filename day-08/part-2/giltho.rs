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

fn find_z(start: usize, map: &[(usize, usize); 26 * 26 * 26], instrs: &[u8]) -> usize {
    let mut pos = start;
    let mut steps = 0;
    loop {
        for &c in instrs {
            if pos % 26 == 25 {
                return steps;
            }
            match c {
                b'L' => pos = map[pos].0,
                _ => pos = map[pos].1,
            }
            steps += 1;
        }
    }
}

fn run(input: &str) -> usize {
    let mut map = [(0usize, 0usize); 26 * 26 * 26];
    let mut lines = input.as_bytes().split(|&c| c == b'\n');
    let instr = lines.next().unwrap();
    let mut in_amount = 0;
    let mut ins = [1; 50];
    lines.next();
    for line in lines {
        unsafe {
            let ent = map_id(&*(line.as_ptr() as *const [u8; 3]));
            if ent % 26 == 0 {
                ins[in_amount] = ent;
                in_amount += 1;
            }
            let left = map_id(&*(line.as_ptr().add(7) as *const [u8; 3]));
            let right = map_id(&*(line.as_ptr().add(12) as *const [u8; 3]));
            *map.get_unchecked_mut(ent) = (left, right);
        }
    }
    ins.into_iter().take(in_amount).fold(1usize, |acc, start| {
        num::integer::lcm(acc, find_z(start, &map, instr))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run(r"LR

AAA = (AAB, XXX)
AAB = (XXX, AAZ)
AAZ = (AAB, XXX)
BBA = (BBB, XXX)
BBB = (BBC, BBC)
BBC = (BBZ, BBZ)
BBZ = (BBB, BBB)
XXX = (XXX, XXX)"),
            6
        )
    }
}
