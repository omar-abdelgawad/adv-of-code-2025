#![allow(warnings)]

use std::fs::read_to_string;

fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let part1_inp = get_part_1_input(filename);
    let ans = part_1(part1_inp);
    dbg!(ans);
}

fn part_1(grid: Vec<Vec<char>>) -> i64 {
    let NEIGHBOURS = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut ans = 0;
    let height = grid.len();
    let width = grid[0].len();
    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == '.' {
                continue;
            }
            let mut num_neighbours = 0;
            for (dh, dw) in &NEIGHBOURS {
                let Ok(i_neighbour) = usize::try_from(i as i64 + dh) else {
                    continue;
                };
                let Ok(j_neighbour) = usize::try_from(j as i64 + dw) else {
                    continue;
                };
                if i_neighbour >= height || j_neighbour >= width {
                    continue;
                }
                if grid[i_neighbour][j_neighbour] == '@' {
                    num_neighbours += 1;
                }
            }
            if num_neighbours < 4 {
                ans += 1;
            }
        }
    }
    ans
}

fn get_part_1_input(filename: &str) -> Vec<Vec<char>> {
    let inp = read_to_string(filename).unwrap();
    parse_input(&inp)
}

fn parse_input(inp: &str) -> Vec<Vec<char>> {
    let mut ret = Vec::new();
    for (i, line) in inp.trim().split("\n").enumerate() {
        ret.push(vec![]);
        for dig in line.chars() {
            ret[i].push(dig);
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_taking_input() {
        let test_example = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        let p1_inp = parse_input(test_example);
        let ans = part_1(p1_inp);
        assert_eq!(ans, 13);
    }
}
