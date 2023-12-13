use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

struct Block<'a> {
    bytes: &'a [u8],
    width: usize,
    height: usize,
}

impl<'a> Block<'a> {
    fn get_line(&self, l: usize) -> &[u8] {
        &self.bytes[l * (self.width + 1)..(l + 1) * (self.width + 1) - 1]
    }

    fn test_h_reflects(&self) -> usize {
        let mut res = 0;
        for h in 1..self.height {
            let mut offset = 1;
            let mut reflects = true;
            while reflects && offset <= h && h + offset - 1 < self.height {
                let upper = self.get_line(h - offset);
                let lower = self.get_line(h + offset - 1);
                reflects = upper == lower;
                offset += 1;
            }
            if reflects {
                res += 100 * h
            }
        }
        res
    }
    fn line_v_reflects(&self, l: usize, w: usize) -> bool {
        let line = self.get_line(l);
        let mut offset = 1;
        let mut reflects = true;
        while reflects && offset <= w && w + offset - 1 < self.width {
            let left = line[w - offset];
            let right = line[w + offset - 1];
            reflects = left == right;
            offset += 1;
        }
        reflects
    }

    fn test_v_reflects(&self) -> usize {
        let mut res = 0;
        for w in 1..self.width {
            let mut reflects = true;
            let mut l = 0;
            while reflects && l < self.height {
                reflects = self.line_v_reflects(l, w);
                l += 1;
            }
            if reflects {
                res += w
            }
        }
        res
    }
}

fn run(input: &str) -> usize {
    let mut res = 0;
    let bytes = input.as_bytes();
    let mut cur = 0;
    let mut start = 0;
    let mut newline = false;
    let mut width = 0;
    let mut height = 0;
    while cur < bytes.len() {
        if bytes[cur] == b'\n' {
            if newline {
                let block = Block {
                    bytes: &bytes[start..cur - 1],
                    width,
                    height,
                };
                res += block.test_h_reflects();
                res += block.test_v_reflects();
                newline = false;
                start = cur + 1;
                width = 0;
                height = 0;
            } else {
                newline = true;
                height += 1;
                if width == 0 {
                    width = cur - start;
                }
            }
        } else {
            // dbg!("parse!", cur);
            newline = false;
        }
        cur += 1;
    }
    if height > 0 {
        let block = Block {
            bytes: &bytes[start..cur - 1],
            width,
            height,
        };
        res += block.test_h_reflects();
        res += block.test_v_reflects();
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
"),
            5
        );
        assert_eq!(
            run("#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"),
            400
        );
        assert_eq!(
            run("#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"),
            405
        );
    }
}
