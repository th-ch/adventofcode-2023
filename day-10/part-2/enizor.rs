use std::env::args;
use std::time::Instant;

use aoc::enizor::bitset::*;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

use aoc::enizor::grid::*;
use Direction::*;

#[derive(Debug, Clone, Copy)]
struct Animal<'a> {
    pos: Position,
    grid: StrGrid<'a>,
}

impl<'a> Animal<'a> {
    fn init(input: &'a str) -> Option<Self> {
        input.as_bytes().iter().position(|b| *b == b'S').map(|idx| {
            let grid = StrGrid::from_input(input);
            Animal {
                pos: grid.from_cur(idx),
                grid,
            }
        })
    }
    fn check_pipe(&self, dir: Direction) -> Option<Direction> {
        let pipe = self.grid[self.pos];
        match (dir, pipe) {
            (Up, b'F') => Some(Right),
            (Up, b'7') => Some(Left),
            (Up, b'|') => Some(Up),
            (Down, b'L') => Some(Right),
            (Down, b'J') => Some(Left),
            (Down, b'|') => Some(Down),
            (Right, b'J') => Some(Up),
            (Right, b'7') => Some(Down),
            (Right, b'-') => Some(Right),
            (Left, b'L') => Some(Up),
            (Left, b'F') => Some(Down),
            (Left, b'-') => Some(Left),
            _ => None,
        }
    }
    fn try_loop(mut self, mut dir: Direction) -> Option<usize> {
        let mut loop_bitset = VecBitSet::new(bitset_size(self.grid.data.len()));
        let start = self.grid.cur(self.pos);
        let start_dir = dir;
        while self.grid.step_mut(&mut self.pos, dir) {
            let idx = self.grid.cur(self.pos);
            loop_bitset.set(idx);
            if idx == start {
                // compute interior: on a given line:
                // start/end in exterior
                // a | means we switch inside / outside
                // a - does nothing
                // a XL---JX or XF---7X stays the same
                // a XL---7Y or XF---JY switches
                // treat both L & J as | and F & 7 as - to obtain the same result
                // compute the equivalent for the start S using the dir-dir_start chain
                let start_toggles = match (dir, start_dir) {
                    (Up, Up) => true,      // |
                    (Down, Down) => true,  // |
                    (Down, Right) => true, // L
                    (Left, Up) => true,    // L
                    (Down, Left) => true,  // J
                    (Right, Up) => true,   // J
                    _ => false,
                };
                let mut count = 0;
                let mut in_interior = false;
                for (i, b) in self.grid.data.iter().enumerate() {
                    if *b == b'\n' {
                        // we should be back at exterior
                        assert!(!in_interior);
                    } else if !loop_bitset.test(i) {
                        if in_interior {
                            count += 1;
                        }
                    } else {
                        match b {
                            b'|' | b'L' | b'J' => in_interior = !in_interior,
                            b'S' if start_toggles => in_interior = !in_interior,
                            _ => {}
                        }
                    }
                }
                return Some(count);
            }
            if let Some(new_dir) = self.check_pipe(dir) {
                dir = new_dir;
            } else {
                return None;
            }
        }
        None
    }
}

fn run(input: &str) -> usize {
    let animal = Animal::init(input).expect("No animal!");
    ALL_DIRECTIONS
        .iter()
        .flat_map(|dir| animal.try_loop(*dir))
        .next()
        .expect("Failed to run along a loop!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("-L|F7
7S-7|
L|7||
-L-J|
L|-JF"),
            1
        );
        assert_eq!(
            run("7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"),
            1
        );
        assert_eq!(
            run("...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."),
            4
        );
        assert_eq!(
            run("..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
.........."),
            4
        );
        assert_eq!(
            run("..........
.F------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--SL--J.
.........."),
            4
        );
        assert_eq!(
            run("..........
.F------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|IIS|II|.
.L--JL--J.
.........."),
            4
        );
        assert_eq!(
            run("..........
.F------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II|SII|.
.L--JL--J.
.........."),
            4
        );
        assert_eq!(
            run("..........
.F------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|LS7F-J|.
.|II||II|.
.L--JL--J.
.........."),
            4
        );
        assert_eq!(
            run("..........
.F------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7FSJ|.
.|II||II|.
.L--JL--J.
.........."),
            4
        );
        assert_eq!(
            run("..........
.S------7.
.|IIIIII|.
.|IIIIII|.
.|IIIIII|.
.|IIF7II|.
.|II||II|.
.L--JL--J.
.........."),
            26
        );
        assert_eq!(
            run("..........
.F------S.
.|IIIIII|.
.|IIIIII|.
.|IIIIII|.
.|IIF7II|.
.|II||II|.
.L--JL--J.
.........."),
            26
        );
        assert_eq!(
            run("..........
.F------7.
.|IIIIII|.
.|IIIIII|.
.|IIIIII|.
.|IIF7II|.
.|II||II|.
.L--JL--S.
.........."),
            26
        );
        assert_eq!(
            run("..........
.F------7.
.|IIIIII|.
.|IIIIII|.
.|IIIIII|.
.|IIF7II|.
.|II||II|.
.S--JL--J.
.........."),
            26
        );
    }
}
