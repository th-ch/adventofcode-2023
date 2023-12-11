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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

const ALL_DIRECTIONS: [Direction; 4] = [North, East, South, West];

#[derive(Debug, Clone, Copy)]
struct Grid<'a> {
    length: usize,
    height: usize,
    bytes: &'a [u8],
}

impl<'a> Grid<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        let l = bytes
            .iter()
            .position(|b| *b == b'\n')
            .expect("No line ending!")
            + 1;
        let h = (bytes.len() + (l - 1)) / l;
        Self {
            length: l,
            height: h,
            bytes,
        }
    }
    fn find_animal(&'a self) -> Option<Position<'a>> {
        self.bytes.iter().position(|b| *b == b'S').map(|idx| {
            let mut animal = Position {
                x: 0,
                y: 0,
                grid: self,
            };
            animal.set_index(idx);
            animal
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Position<'a> {
    x: usize,
    y: usize,
    grid: &'a Grid<'a>,
}

impl<'a> Position<'a> {
    fn as_index(&self) -> usize {
        self.x + self.grid.length * self.y
    }
    fn set_index(&mut self, idx: usize) {
        self.x = idx % self.grid.length;
        self.y = idx / self.grid.length;
    }
    fn advance(&mut self, dir: Direction) -> Option<usize> {
        let max_y = self.grid.height - 1;
        let max_x = self.grid.length - 2;
        match (dir, self.x, self.y) {
            (North, _, 0) => None,
            (North, _, _) => {
                self.y -= 1;
                Some(self)
            }
            (South, _, y) if y == max_y => None,
            (South, _, _) => {
                self.y += 1;
                Some(self)
            }
            (West, _, 0) => None,
            (West, _, _) => {
                self.x -= 1;
                Some(self)
            }
            (East, _, x) if x == max_x => None,
            (East, _, _) => {
                self.x += 1;
                Some(self)
            }
        }
        .map(|p| p.as_index())
    }
    fn check_pipe(&self, dir: Direction) -> Option<Direction> {
        let pipe = self.grid.bytes[self.as_index()];
        match (dir, pipe) {
            (North, b'F') => Some(East),
            (North, b'7') => Some(West),
            (North, b'|') => Some(North),
            (South, b'L') => Some(East),
            (South, b'J') => Some(West),
            (South, b'|') => Some(South),
            (East, b'J') => Some(North),
            (East, b'7') => Some(South),
            (East, b'-') => Some(East),
            (West, b'L') => Some(North),
            (West, b'F') => Some(South),
            (West, b'-') => Some(West),
            _ => None,
        }
    }
    fn try_loop(mut self, mut dir: Direction) -> Option<usize> {
        let mut loop_bitset = VecBitSet::new(bitset_size(self.grid.bytes.len()));
        let start = self.as_index();
        let start_dir = dir;
        while let Some(idx) = self.advance(dir) {
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
                    (North, North) => true, // |
                    (South, South) => true, // |
                    (South, East) => true,  // L
                    (West, North) => true,  // L
                    (South, West) => true,  // J
                    (East, North) => true,  // J
                    _ => false,
                };
                let mut count = 0;
                let mut in_interior = false;
                for (i, b) in self.grid.bytes.iter().enumerate() {
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
    let grid = Grid::new(input.as_bytes());
    let animal = grid.find_animal().expect("No animal!");
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
