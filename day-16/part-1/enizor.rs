use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum Direction {
    Left = 0b1,
    Right = 0b10,
    Up = 0b100,
    Down = 0b1000,
}
use Direction::*;

struct Contraption<'a> {
    grid: &'a [u8],
    grid_beams: Vec<u8>,
    width: usize,
    height: usize,
}

impl<'a> Contraption<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        let width = bytes
            .iter()
            .position(|b| *b == b'\n')
            .expect("No newline in input!");
        let height = (bytes.len() + 1) / (width + 1);
        Self {
            grid: bytes,
            width,
            height,
            grid_beams: vec![0; bytes.len()],
        }
    }
    fn get(&mut self, x: usize, y: usize) -> (&u8, &mut u8) {
        let cur = x + y * (self.width + 1);
        (&self.grid[cur], &mut self.grid_beams[cur])
    }
    // moves (x,y) accroding to Direction.
    // returns true if stepped out of bounds
    fn step(&self, x: &mut usize, y: &mut usize, dir: Direction) -> bool {
        match (dir, *x, *y) {
            (Left, 0, _) => return true,
            (Right, _, _) if *x == self.width - 1 => return true,
            (Up, _, 0) => return true,
            (Down, _, _) if *y == self.height - 1 => return true,
            (Left, _, _) => *x -= 1,
            (Right, _, _) => *x += 1,
            (Up, _, _) => *y -= 1,
            (Down, _, _) => *y += 1,
        }
        false
    }
    fn start_beam(&mut self) -> usize {
        let mut energized = 0;
        let mut beams = Vec::with_capacity(16);
        beams.push((0, 0, Right));
        while let Some((mut x, mut y, mut dir)) = beams.pop() {
            let (ground, seen_beams) = self.get(x, y);
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
                    let (mut x2, mut y2, dir2) = (x, y, Right);
                    if !self.step(&mut x2, &mut y2, dir2) {
                        beams.push((x2, y2, dir2));
                    }
                }
                (b'|', Left) | (b'|', Right) => {
                    dir = Down;
                    let (mut x2, mut y2, dir2) = (x, y, Up);
                    if !self.step(&mut x2, &mut y2, dir2) {
                        beams.push((x2, y2, dir2));
                    }
                }
                _ => {}
            }
            if !self.step(&mut x, &mut y, dir) {
                beams.push((x, y, dir));
            }
        }
        energized
    }
}

fn run(input: &str) -> usize {
    let mut contraption = Contraption::new(input.as_bytes());
    contraption.start_beam()
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
..//.|...."),
            46
        )
    }
}
