use aoc::enizor::bitset::{bitset_size, VecBitSet};
use aoc::enizor::grid::*;
use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

/// values of heat loss for different _outbound_ directions Horizontal/Vertical
/// i.e. this value was reached using Vertical/Horizontal
#[derive(Debug, Clone)]
struct HeatLoss {
    values: [usize; 2],
}
use Direction::*;

impl std::ops::Index<Direction> for HeatLoss {
    type Output = usize;

    fn index(&self, dir: Direction) -> &Self::Output {
        match dir {
            Left => &self.values[0],
            Right => &self.values[0],
            Up => &self.values[1],
            Down => &self.values[1],
        }
    }
}

impl std::ops::IndexMut<Direction> for HeatLoss {
    fn index_mut(&mut self, dir: Direction) -> &mut Self::Output {
        match dir {
            Left => &mut self.values[0],
            Right => &mut self.values[0],
            Up => &mut self.values[1],
            Down => &mut self.values[1],
        }
    }
}

impl std::default::Default for HeatLoss {
    fn default() -> Self {
        Self {
            values: [usize::MAX; 2],
        }
    }
}

struct HeatLossMap<'a> {
    grid: StrGrid<'a>,
    heat_loss: Vec<HeatLoss>,
}

impl<'a> HeatLossMap<'a> {
    fn new(input: &'a str) -> Self {
        let grid = StrGrid::from_input(input);
        Self {
            heat_loss: vec![HeatLoss::default(); grid.data.len()],
            grid,
        }
    }
    fn compute_loss(&mut self) {
        let mut positions = VecBitSet::new(bitset_size(self.grid.data.len()));
        positions.set(0usize);
        while let Some(cur) = positions.first_set() {
            positions.reset(cur);
            let p = self.grid.from_cur(cur);
            for out_dir in ALL_DIRECTIONS {
                let mut pos = p;
                let current_loss = self.heat_loss[cur][out_dir];
                if current_loss == usize::MAX {
                    continue;
                }
                let mut new_loss = current_loss;
                for step in 0..10 {
                    if self.grid.step_mut(&mut pos, out_dir) {
                        let new_cur = self.grid.cur(pos);
                        new_loss += (self.grid[pos] - b'0') as usize;
                        if step >= 3 {
                            let new_dirs = if (out_dir as u8) < 3 {
                                [Up, Down]
                            } else {
                                [Left, Right]
                            };
                            for new_dir in new_dirs {
                                let old_loss = &mut self.heat_loss[new_cur][new_dir];
                                if *old_loss > new_loss {
                                    *old_loss = new_loss;
                                    positions.set(new_cur)
                                }
                            }
                        }
                    } else {
                        // stepped out of bounds
                        break;
                    }
                }
            }
        }
    }
}

fn run(input: &str) -> usize {
    let mut map = HeatLossMap::new(input);
    let start_corner = map.grid.cur(Position { x: 0, y: 0 });
    map.heat_loss[start_corner] = HeatLoss { values: [0; 2] };
    map.compute_loss();
    let end_corner = map.grid.cur(Position {
        x: map.grid.width - 2,
        y: map.grid.height - 1,
    });

    *map.heat_loss[end_corner].values.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("111111111111
999999999991
999999999991
999999999991
999999999991"),
            71
        );
        assert_eq!(
            run("2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"),
            94
        );
    }
}
