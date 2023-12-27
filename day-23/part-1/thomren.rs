use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> isize {
    let grid: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let start = (0, 1);
    let end = (grid.len() - 1, grid[0].len() - 2);

    let mut stack = vec![(start, 0)];
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut max = 0;
    while let Some((pos, dist)) = stack.pop() {
        // println!("{:?} {}", pos, dist);
        if pos == end {
            max = max.max(dist);
            continue;
        }
        if dist == -1 {
            visited[pos.0][pos.1] = false;
            continue;
        }
        if visited[pos.0][pos.1] {
            continue;
        }
        visited[pos.0][pos.1] = true;

        // will backtrack once all the paths from pos have been explored
        stack.push((pos, -1));

        for neighbor in get_neighbors(&grid, pos) {
            if !visited[neighbor.0][neighbor.1] {
                stack.push((neighbor, dist + 1));
            }
        }
    }
    max
}

fn get_neighbors(grid: &Vec<&[u8]>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    if pos.0 > 0 && (grid[pos.0 - 1][pos.1] == b'.' || grid[pos.0 - 1][pos.1] == b'^') {
        neighbors.push((pos.0 - 1, pos.1));
    }
    if pos.0 < grid.len() - 1 && (grid[pos.0 + 1][pos.1] == b'.' || grid[pos.0 + 1][pos.1] == b'v')
    {
        neighbors.push((pos.0 + 1, pos.1));
    }
    if pos.1 > 0 && (grid[pos.0][pos.1 - 1] == b'.' || grid[pos.0][pos.1 - 1] == b'<') {
        neighbors.push((pos.0, pos.1 - 1));
    }
    if pos.1 < grid[0].len() - 1
        && (grid[pos.0][pos.1 + 1] == b'.' || grid[pos.0][pos.1 + 1] == b'>')
    {
        neighbors.push((pos.0, pos.1 + 1));
    }
    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"),
            94
        )
    }
}
