use aoc::enizor::grid::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

use Direction::*;

const DIRECTIONS_BY_AXIS: [[Direction; 2]; 2] = [[Up, Down], [Left, Right]];

struct HeatLossMap<'a> {
    grid: StrGrid<'a>,
    heat_loss: Vec<[usize; 2]>,
}

impl<'a> HeatLossMap<'a> {
    fn new(input: &'a str) -> Self {
        let grid = StrGrid::from_input(input);
        Self {
            heat_loss: vec![[usize::MAX; 2]; grid.data.len()],
            grid,
        }
    }

    fn compute_loss(&mut self) -> usize {
        let mut queue = BinaryHeap::with_capacity(self.grid.width);
        queue.push((Reverse(0), Reverse(0), Position::default(), false));
        queue.push((Reverse(0), Reverse(0), Position::default(), true));
        let goal = Position {
            x: self.grid.width - 2,
            y: self.grid.height - 1,
        };
        while let Some((Reverse(_heuristic), Reverse(cost), p, vertical)) = queue.pop() {
            if p == goal {
                return cost;
            }
            let out = DIRECTIONS_BY_AXIS[vertical as usize];
            for out_dir in out {
                let mut pos = p;
                let current_loss = cost;
                let mut new_loss = current_loss;
                for step in 0..10 {
                    if self.grid.step_mut(&mut pos, out_dir) {
                        new_loss += (self.grid[pos] - b'0') as usize;
                        if step >= 3 {
                            let new_cur = self.grid.cur(pos);
                            let h = pos.x.abs_diff(goal.x) + pos.y.abs_diff(goal.y);
                            let new_vert = !vertical;
                            let old_loss = &mut self.heat_loss[new_cur][new_vert as usize];
                            if *old_loss > new_loss {
                                *old_loss = new_loss;
                                queue.push((
                                    Reverse(new_loss + h),
                                    Reverse(new_loss),
                                    pos,
                                    new_vert,
                                ));
                            }
                        }
                    } else {
                        // stepped out of bounds
                        break;
                    }
                }
            }
        }
        panic!("Failed to reach the goal!")
    }
}

fn run(input: &str) -> usize {
    let mut map = HeatLossMap::new(input);
    map.compute_loss()
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
