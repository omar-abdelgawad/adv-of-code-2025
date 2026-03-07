#![allow(warnings)]

use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let part1_inp = get_part_1_input(filename);
    let ans = part_1(part1_inp);
    dbg!(ans);
}

fn part_1((grid): (Vec<Vec<char>>)) -> i64 {
    let mut ans = 0;
    let first_ind = grid[0].iter().position(|&c| c == 'S').unwrap();
    let mut first_set = HashSet::from([first_ind]);
    let mut new_set = HashSet::new();
    for i in 1..grid.len() {
        for ray_ind in first_set {
            if grid[i][ray_ind] == '^' {
                ans += 1;
                new_set.insert(ray_ind + 1);
                new_set.insert(ray_ind - 1);
            } else {
                new_set.insert(ray_ind);
            }
        }
        first_set = new_set.clone();
        new_set.clear();
    }
    ans
}

fn get_part_1_input(filename: &str) -> (Vec<Vec<char>>) {
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
        let ans = part_1(p1_inp);
        assert_eq!(ans, 21);
    }
}
