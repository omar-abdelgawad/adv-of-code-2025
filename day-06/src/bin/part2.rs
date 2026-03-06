#![allow(warnings)]

use std::fs::read_to_string;

fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let part2_inp = get_part_2_input(filename);
    let ans = part_2(part2_inp);
    dbg!(ans);
}

fn part_2((grid): Vec<Vec<char>>) -> i64 {
    let mut ans = 0;
    let mut height = grid.len();
    let mut width = grid[0].len();
    let mut new_column = vec![];
    for j in (0..width).rev() {
        let mut new_num = String::new();
        for i in 0..height - 1 {
            if grid[i][j].is_numeric() {
                new_num.push(grid[i][j]);
            }
        }
        if let Ok(new_num) = new_num.parse::<i64>() {
            new_column.push(new_num);
        }
        match grid[height - 1][j] {
            '+' => ans += new_column.drain(..).sum::<i64>(),
            '*' => ans += new_column.drain(..).product::<i64>(),
            _ => {}
        }
    }
    ans
}

fn get_part_2_input(filename: &str) -> Vec<Vec<char>> {
    let inp = read_to_string(filename).unwrap();
    parse_input(&inp)
}

fn parse_input(inp: &str) -> Vec<Vec<char>> {
    inp.lines().map(|l| l.chars().collect()).collect()
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
        let ans = part_2(p1_inp);
        assert_eq!(ans, 3263827);
    }
}
