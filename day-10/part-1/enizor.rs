use std::env::args;
use std::time::Instant;

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
    /// Takes self by value so that calls use copies of self
    fn try_loop(mut self, mut dir: Direction) -> Option<usize> {
        let start = self.pos;
        let mut length = 0;
        while self.grid.step_mut(&mut self.pos, dir) {
            length += 1;
            if self.pos == start {
                return Some(length / 2);
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
            4
        );
        assert_eq!(
            run("7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"),
            8
        );
    }
}
