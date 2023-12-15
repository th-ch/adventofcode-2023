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
    let mut res = 0;
    let mut hash = 0;
    for b in input.bytes() {
        match b {
            b'\n' => {}
            b',' => {
                res += hash;
                hash = 0;
            }
            _ => {
                hash += b as usize;
                hash *= 17;
                hash &= 0xFF;
            }
        }
    }
    // Your code goes here
    res + hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(run("HASH"), 52);
        assert_eq!(
            run("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320
        );
    }
}
