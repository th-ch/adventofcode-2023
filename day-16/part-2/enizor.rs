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

#[derive(Debug, Clone)]
struct Contraption<'a> {
    grid_beams: Vec<u8>,
    grid: StrGrid<'a>,
}

impl<'a> Contraption<'a> {
    fn new(input: &'a str) -> Self {
        let grid = StrGrid::from_input(input);
        dbg!(&grid);
        Self {
            grid_beams: vec![0; grid.data.len()],
            grid,
        }
    }
    fn get(&mut self, pos: Position) -> (&u8, &mut u8) {
        let cur = self.grid.cur(pos);
        (&self.grid.data[cur], &mut self.grid_beams[cur])
    }
    fn reset_beams(&mut self) {
        self.grid_beams.fill(0);
    }
    fn start_beam(&mut self, start: (Position, Direction)) -> usize {
        let mut energized = 0;
        let mut beams = Vec::with_capacity(16);
        beams.push(start);
        while let Some((mut pos, mut dir)) = beams.pop() {
            let (ground, seen_beams) = self.get(pos);
            if *seen_beams & (dir as u8) != 0 {
                // we already went here in this direction
                continue;
            }
            if *seen_beams == 0 {
                energized += 1;
            }
            *seen_beams |= dir as u8;
            match (*ground, dir) {
                (b'/', Left) => dir = Down,
                (b'/', Right) => dir = Up,
                (b'/', Down) => dir = Left,
                (b'/', Up) => dir = Right,
                (b'\\', Left) => dir = Up,
                (b'\\', Right) => dir = Down,
                (b'\\', Down) => dir = Right,
                (b'\\', Up) => dir = Left,
                (b'-', Up) | (b'-', Down) => {
                    dir = Left;
                    let mut pos2 = pos;
                    let dir2 = Right;
                    if self.grid.step_mut(&mut pos2, dir2) {
                        beams.push((pos2, dir2));
                    }
                }
                (b'|', Left) | (b'|', Right) => {
                    dir = Down;
                    let mut pos2 = pos;
                    let dir2 = Up;
                    if self.grid.step_mut(&mut pos2, dir2) {
                        beams.push((pos2, dir2));
                    }
                }
                _ => {}
            }
            if self.grid.step_mut(&mut pos, dir) {
                beams.push((pos, dir));
            }
        }
        energized
    }
}

fn run(input: &str) -> usize {
    let mut contraption = Contraption::new(input);
    let mut max = 0;
    for x in 0..contraption.grid.width - 1 {
        max = max.max(contraption.start_beam((Position { x, y: 0 }, Down)));
        contraption.reset_beams();
        max = max.max(contraption.start_beam((
            Position {
                x,
                y: contraption.grid.height - 1,
            },
            Up,
        )));
        contraption.reset_beams();
    }
    for y in 0..contraption.grid.height {
        max = max.max(contraption.start_beam((Position { x: 0, y }, Right)));
        contraption.reset_beams();
        max = max.max(contraption.start_beam((
            Position {
                x: contraption.grid.width - 2,
                y,
            },
            Left,
        )));
        contraption.reset_beams();
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run(r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"),
            51
        )
    }
}
