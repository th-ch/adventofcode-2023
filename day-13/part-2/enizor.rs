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

    fn test_h_reflects_smudged(&self) -> usize {
        let mut res = 0;
        for h in 1..self.height {
            let mut offset = 1;
            let mut errors = 0;
            while errors < 2 && offset <= h && h + offset - 1 < self.height {
                let upper = self.get_line(h - offset);
                let lower = self.get_line(h + offset - 1);
                let mut c = 0;
                while errors < 2 && c < self.width {
                    if upper[c] != lower[c] {
                        errors += 1;
                    }
                    c += 1;
                }
                offset += 1;
            }
            if errors == 1 {
                res += 100 * h
            }
        }
        res
    }

    // error counter: 0- is perfect reflection, 1 is smudged
    fn line_v_reflects_errors(&self, l: usize, w: usize) -> usize {
        let line = self.get_line(l);
        let mut offset = 1;
        let mut errors = 0;
        while errors < 2 && offset <= w && w + offset - 1 < self.width {
            let left = line[w - offset];
            let right = line[w + offset - 1];
            if left != right {
                errors += 1;
            };
            offset += 1;
        }
        errors
    }

    fn test_v_reflects_smudged(&self) -> usize {
        let mut res = 0;
        for w in 1..self.width {
            let mut errors = 0;
            let mut l = 0;
            while errors < 2 && l < self.height {
                errors += self.line_v_reflects_errors(l, w);
                l += 1;
            }
            if errors == 1 {
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
                res += block.test_h_reflects_smudged();
                res += block.test_v_reflects_smudged();
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
        res += block.test_h_reflects_smudged();
        res += block.test_v_reflects_smudged();
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
            300
        );
        assert_eq!(
            run("#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"),
            100
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
            400
        );
    }
}
