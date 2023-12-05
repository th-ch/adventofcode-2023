use std::collections::HashMap;
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
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let grid_length: usize = grid.len();
    let grid_width: usize = grid[0].len();

    let mut gear_map: HashMap<[usize; 2], Vec<usize>> = HashMap::new();

    for i in 0..grid_length {
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
                    [i.saturating_sub(1)..=(grid_length - 1).min(i + 1)]
                    .iter()
                    .map(|row| row[j.saturating_sub(n_cursor + 1)..=j].to_vec())
                    .collect();

                let mut gear_pos: [usize; 2] = [0, 0];
                for (k, row) in adjacent.iter().enumerate() {
                    for (l, c) in row.iter().enumerate() {
                        if !c.is_digit(10) && *c == '*' {
                            if i == 0 {
                                gear_pos[0] = k;
                            } else {
                                gear_pos[0] = k + i - 1;
                            }
                            if j - n_cursor == 0 {
                                gear_pos[1] = l + j - n_cursor;
                            } else {
                                gear_pos[1] = l + j - n_cursor - 1;
                            }
                            if gear_map.contains_key(&gear_pos) {
                                gear_map.get_mut(&gear_pos).unwrap().push(part_num as usize);
                            } else {
                                gear_map.insert(gear_pos, vec![part_num as usize]);
                            }
                        }
                    }
                }
                n_cursor = 0;
            }
        }
    }
    let mut gear_ratio: usize = 0;
    for (_, v) in gear_map.iter() {
        if v.len() == 2 {
            gear_ratio += v.iter().product::<usize>();
        }
    }
    gear_ratio
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
            467835
        )
    }

    #[test]
    fn same_line() {
        assert_eq!(
            run("..2.3
.*11*"),
            55
        )
    }

    #[test]
    fn overlap() {
        assert_eq!(
            run(".002.
*.*.*
.005."),
            3 * 2 * 5
        );
    }

    #[test]
    fn overlap2() {
        assert_eq!(
            run("2.3
.*.
5.7
.*."),
            35
        );
    }
}
