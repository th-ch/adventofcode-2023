use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

macro_rules! update {
    ($first:ident, $last:ident, $n:expr) => {
        $last = $n;
        if $first == -1 {
            $first = $last;
        }
        break;
    };

    ($last:ident, $n:expr) => {
        $last = $n;
        break;
    };
}

fn run(input: &str) -> isize {
    let mut sum = 0;
    for line in input.as_bytes().split(|x| *x == b'\n') {
        let mut first = -1;
        let mut last = -1;
        let mut i = 0;
        loop {
            match unsafe { line.get_unchecked(i) } {
                c @ (b'0'..=b'9') => {
                    update!(first, last, (*c - b'0') as isize);
                }
                b'o' if line.len() - i >= 3 && &line[i..i + 3] == b"one".as_slice() => {
                    update!(first, last, 1);
                }
                b't' if line.len() - i >= 3 && &line[i..i + 3] == b"two".as_slice() => {
                    update!(first, last, 2);
                }
                b't' if line.len() - i >= 5 && &line[i..i + 5] == b"three".as_slice() => {
                    update!(first, last, 3);
                }
                b'f' if line.len() - i >= 4 && &line[i..i + 4] == b"four".as_slice() => {
                    update!(first, last, 4);
                }
                b'f' if line.len() - i >= 4 && &line[i..i + 4] == b"five".as_slice() => {
                    update!(first, last, 5);
                }
                b's' if line.len() - i >= 3 && &line[i..i + 3] == b"six".as_slice() => {
                    update!(first, last, 6);
                }
                b's' if line.len() - i >= 5 && &line[i..i + 5] == b"seven".as_slice() => {
                    update!(first, last, 7);
                }
                b'e' if line.len() - i >= 5 && &line[i..i + 5] == b"eight".as_slice() => {
                    update!(first, last, 8);
                }
                b'n' if line.len() - i >= 4 && &line[i..i + 4] == b"nine".as_slice() => {
                    update!(first, last, 9);
                }
                _ => (),
            }
            i += 1;
        }
        i = line.len() - 1;
        loop {
            match unsafe { line.get_unchecked(i) } {
                c @ (b'0'..=b'9') => {
                    update!(first, last, (*c - b'0') as isize);
                }
                b'o' if line.len() - i >= 3 && &line[i..i + 3] == b"one".as_slice() => {
                    update!(first, last, 1);
                }
                b't' if line.len() - i >= 3 && &line[i..i + 3] == b"two".as_slice() => {
                    update!(first, last, 2);
                }
                b't' if line.len() - i >= 5 && &line[i..i + 5] == b"three".as_slice() => {
                    update!(first, last, 3);
                }
                b'f' if line.len() - i >= 4 && &line[i..i + 4] == b"four".as_slice() => {
                    update!(first, last, 4);
                }
                b'f' if line.len() - i >= 4 && &line[i..i + 4] == b"five".as_slice() => {
                    update!(first, last, 5);
                }
                b's' if line.len() - i >= 3 && &line[i..i + 3] == b"six".as_slice() => {
                    update!(first, last, 6);
                }
                b's' if line.len() - i >= 5 && &line[i..i + 5] == b"seven".as_slice() => {
                    update!(first, last, 7);
                }
                b'e' if line.len() - i >= 5 && &line[i..i + 5] == b"eight".as_slice() => {
                    update!(first, last, 8);
                }
                b'n' if line.len() - i >= 4 && &line[i..i + 4] == b"nine".as_slice() => {
                    update!(first, last, 9);
                }
                _ => (),
            }
            i -= 1;
        }
        sum += first * 10 + last;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run(r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"),
            281
        )
    }
}
