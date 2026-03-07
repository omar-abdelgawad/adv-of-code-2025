#![allow(warnings)]

use std::fs::read_to_string;

fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let part2_inp = get_part_2_input(filename);
    let ans = part_2(part2_inp);
    dbg!(ans);
}

fn part_2((grid): (Vec<Vec<char>>)) -> u64 {
    let first_ind = grid[0].iter().position(|&c| c == 'S').unwrap();
    let row_len = grid[0].len();
    let height_len = grid.len();
    let mut dp = vec![vec![0u64; row_len]; height_len];
    dp[1][first_ind] = 1;
    for i in 1..height_len - 1 {
        for j in 0..row_len {
            if grid[i + 1][j] == '^' {
                dp[i + 1][j - 1] += dp[i][j];
                dp[i + 1][j + 1] += dp[i][j];
            } else {
                dp[i + 1][j] += dp[i][j];
            }
        }
    }
    dp.pop().unwrap().iter().sum()
}

fn get_part_2_input(filename: &str) -> (Vec<Vec<char>>) {
    let inp = read_to_string(filename).unwrap();
    parse_input(&inp)
}

fn parse_input(inp: &str) -> (Vec<Vec<char>>) {
    inp.trim().lines().map(|l| l.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_taking_input() {
        let test_example = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

        let p1_inp = parse_input(test_example);
        let ans = part_2(p1_inp);
        assert_eq!(ans, 40);
    }
}
