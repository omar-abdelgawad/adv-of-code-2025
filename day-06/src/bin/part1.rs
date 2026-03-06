#![allow(warnings)]

use std::fs::read_to_string;

fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let part1_inp = get_part_1_input(filename);
    let ans = part_1(part1_inp);
    dbg!(ans);
}

fn part_1((grid, operations): (Vec<Vec<i64>>, Vec<char>)) -> i64 {
    //dbg!(&grid, &operations);
    let mut ans = 0;
    for (i, operator) in operations.iter().enumerate() {
        let mut column = vec![];
        for ind in 0..grid.len() {
            column.push(grid[ind][i]);
        }
        match operator {
            '+' => ans += column.iter().sum::<i64>(),
            '*' => ans += column.iter().product::<i64>(),
            _ => panic!(),
        }
    }
    ans
}

fn get_part_1_input(filename: &str) -> (Vec<Vec<i64>>, Vec<char>) {
    let inp = read_to_string(filename).unwrap();
    parse_input(&inp)
}

fn parse_input(inp: &str) -> (Vec<Vec<i64>>, Vec<char>) {
    let mut grid = Vec::new();
    let mut operations = Vec::new();
    let mut iter = inp.trim().split('\n');
    let operations_str = iter.next_back().unwrap().trim();
    for operator in operations_str.trim().split_whitespace() {
        operations.push(operator.chars().next().unwrap());
    }
    for line in iter {
        let line = line.trim();
        let line_num = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        grid.push(line_num);
    }
    (grid, operations)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_taking_input() {
        let test_example = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + 
";

        let p1_inp = parse_input(test_example);
        let ans = part_1(p1_inp);
        assert_eq!(ans, 4277556);
    }
}
