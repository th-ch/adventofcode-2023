use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

#[derive(Clone)]
struct DishPlatform {
    bytes: Vec<u8>,
    height: usize,
    width: usize,
}

impl DishPlatform {
    fn new(bytes: &[u8]) -> Self {
        let width = bytes
            .iter()
            .position(|b| *b == b'\n')
            .expect("No newiline in input!");
        let height = (bytes.len() + 1) / (width + 1);
        DishPlatform {
            bytes: bytes.to_vec(),
            width,
            height,
        }
    }
    #[allow(dead_code)]
    fn debug(&self) -> String {
        format!(
            "w: {} ; h: {} :\n{}\n",
            self.width,
            self.height,
            aoc::enizor::utils::debug_ascii(&self.bytes)
        )
    }
    fn shift_north(&mut self) {
        let mut x = 0;
        let mut space_north = vec![0; self.width];
        let mut h = 0;
        let mut cur = 0;
        while cur < self.bytes.len() {
            let b = self.bytes[cur];
            if b == b'\n' {
                x = 0;
                h += 1;
                cur += 1;
                continue;
            }
            match b {
                b'.' => {}
                b'#' => {
                    space_north[x] = h + 1;
                }
                b'O' => {
                    if space_north[x] < h {
                        let dh = h - space_north[x];
                        self.bytes[cur - (self.width + 1) * dh] = b'O';
                        self.bytes[cur] = b'.';
                    }
                    space_north[x] += 1;
                }
                _ => panic!("Unexpected char {}", b as char),
            }
            x += 1;
            cur += 1;
        }
    }
    fn shift_south(&mut self) {
        let mut x = self.width;
        let mut space_south = vec![self.height; self.width];
        let mut h = self.height;
        let mut cur = (self.width + 1) * self.height - 1;
        assert!(self.bytes[cur - 1] != b'\n');
        while cur > 0 {
            cur -= 1;
            let b = self.bytes[cur];
            if b == b'\n' {
                x = self.width;
                h -= 1;
                continue;
            }
            x -= 1;
            match b {
                b'.' => {}
                b'#' => {
                    space_south[x] = h - 1;
                }
                b'O' => {
                    if space_south[x] > h {
                        let dh = space_south[x] - h;
                        self.bytes[cur + (self.width + 1) * dh] = b'O';
                        self.bytes[cur] = b'.';
                    }
                    space_south[x] -= 1;
                }
                _ => panic!("Unexpected char {}", b as char),
            }
        }
    }
    fn shift_west(&mut self) {
        // println!("{}", self.debug());
        let mut x = 0;
        let mut space_west = 0;
        let mut cur = 0;
        while cur < self.bytes.len() {
            let b = self.bytes[cur];
            if b == b'\n' {
                x = 0;
                cur += 1;
                space_west = 0;
                continue;
            }
            match b {
                b'.' => {}
                b'#' => {
                    space_west = x + 1;
                }
                b'O' => {
                    if space_west < x {
                        let dx = x - space_west;
                        self.bytes[cur - dx] = b'O';
                        self.bytes[cur] = b'.';
                    }
                    space_west += 1;
                }
                _ => panic!("Unexpected char {}", b as char),
            }
            x += 1;
            cur += 1;
        }
        // println!("{}", self.debug());
    }
    fn shift_east(&mut self) -> usize {
        let mut x = self.width;
        let mut space_east = self.width;
        let mut cur = (self.width + 1) * self.height - 1;
        let mut load = 0;
        let mut h = 1;
        while cur > 0 {
            cur -= 1;
            let b = self.bytes[cur];
            if b == b'\n' {
                x = self.width;
                space_east = self.width;
                h += 1;
                continue;
            }
            match b {
                b'.' => {}
                b'#' => {
                    space_east = x - 1;
                }
                b'O' => {
                    load += h;
                    if space_east > x {
                        let dx = space_east - x;
                        self.bytes[cur + dx] = b'O';
                        self.bytes[cur] = b'.';
                    }
                    space_east -= 1;
                }
                _ => panic!("Unexpected char {}", b as char),
            }
            x -= 1;
        }
        load
    }
    fn cycle(&mut self) -> usize {
        self.shift_north();
        self.shift_west();
        self.shift_south();
        self.shift_east()
    }
    fn cycle_loop(mut self, max: usize) -> usize {
        let mut cycles = Vec::new();
        let mut load;
        load = self.cycle();
        cycles.push((self.bytes.clone(), load));
        for k in 1..max {
            load = self.cycle();
            if let Some(pos) = cycles
                .iter()
                .position(|x| x.1 == load && *x.0 == self.bytes)
            {
                let cycle_len = k - pos;
                let offset = (max - k - 1) % cycle_len;
                return cycles[pos + offset].1;
            } else {
                cycles.push((self.bytes.clone(), load));
            }
        }
        load
    }
}

fn run(input: &str) -> usize {
    let platform = DishPlatform::new(input.as_bytes());
    platform.cycle_loop(1000000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_test() {
        let init = b"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let mut p = DishPlatform::new(init);
        let res_north = b"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";
        p.shift_north();
        assert_eq!(&p.bytes, res_north);
        let res_west = b"OOOO.#O...
OO..#....#
OOO..##O..
O..#OO....
........#.
..#....#.#
O....#OO..
O.........
#....###..
#....#....";
        p.shift_west();
        assert_eq!(&p.bytes, res_west);
        let res_south = b".....#....
....#.O..#
O..O.##...
O.O#......
O.O....O#.
O.#..O.#.#
O....#....
OO....OO..
#O...###..
#O..O#....";
        p.shift_south();
        assert_eq!(&p.bytes, res_south);
        let res_east = b".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";
        let load = p.shift_east();
        assert_eq!(&p.bytes, res_east);
        assert_eq!(load, 2 + 2 + 12 + 4 + 10 + 18 + 14 + 16 + 9);
    }

    #[test]
    fn cycle_test() {
        let init = b"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let mut p = DishPlatform::new(init);
        p.cycle();
        let res1 = b".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";
        assert_eq!(&p.bytes, res1);
        p.cycle();
        let res2 = b".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O";
        assert_eq!(&p.bytes, res2);
        p.cycle();
        let res3 = b".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";
        assert_eq!(&p.bytes, res3);
    }

    #[test]
    fn run_test() {
        assert_eq!(
            run("O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."),
            64
        )
    }
}
