use std::env::args;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let output = run(&args().nth(1).expect("Please provide an input"));
    let elapsed = now.elapsed();
    println!("_duration:{}", elapsed.as_secs_f64() * 1000.);
    println!("{}", output);
}

fn run(input: &str) -> usize {
    // Your code goes here
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let grid_lenght: usize = grid.len();
    let grid_width: usize = grid[0].len();

    let mut part_num_sum: usize = 0;
    for i in 0..grid_lenght {
        let mut n_cursor: usize = 0;
        for j in 0..grid_width {
            if grid[i][j].is_digit(10) {
                n_cursor += 1;
                if j != grid_width - 1 {
                    continue;
                }
            }
            let mut part_num: u32 = 0;
            if n_cursor > 0 {
                if (j == grid_width - 1) && grid[i][j].is_digit(10) {
                    n_cursor -= 1;
                    for k in 0..n_cursor + 1 {
                        part_num += grid[i][j - k].to_digit(10).unwrap() * 10u32.pow(k as u32);
                    }
                } else {
                    for k in 0..n_cursor {
                        part_num += grid[i][j - k - 1].to_digit(10).unwrap() * 10u32.pow(k as u32);
                    }
                }

                let adjacent: Vec<Vec<char>> = grid
                    [i.saturating_sub(1)..=(grid_lenght - 1).min(i + 1)]
                    .iter()
                    .map(|row| row[j.saturating_sub(n_cursor + 1)..=j].to_vec())
                    .collect();

                let mut is_part_num: bool = false;
                for row in adjacent.iter() {
                    for c in row.iter() {
                        if !c.is_digit(10) && *c != '.' {
                            is_part_num = true;
                            break;
                        }
                    }
                }
                n_cursor = 0;
                if is_part_num {
                    part_num_sum += part_num as usize;
                }
            }
        }
    }
    part_num_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(
            run("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."),
            4361
        )
    }

    #[test]
    fn in_border() {
        assert_eq!(
            run("..11
11*.."),
            22
        )
    }
}
