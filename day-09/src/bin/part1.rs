#![allow(warnings)]

use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let part1_inp = get_part_1_input(filename);
    let ans = part_1(part1_inp);
    dbg!(ans);
}

// could be u64 but then I have to carefully handle subtraction
type Point = (i64, i64);
type PIndex = usize;

fn part_1(points: Vec<Point>) -> i64 {
    let mut ans = 0;
    let num_points = points.len();
    for i in 0..num_points - 1 {
        for j in i + 1..num_points {
            ans = ans.max(area(&points[i], &points[j]))
        }
    }
    ans
}

fn area(p1: &Point, p2: &Point) -> i64 {
    ((p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1)) as i64
}

fn get_part_1_input(filename: &str) -> Vec<Point> {
    let inp = read_to_string(filename).unwrap();
    parse_input(&inp)
}

fn parse_input(inp: &str) -> Vec<Point> {
    let mut ret = vec![];
    for line in inp.trim().lines() {
        let mut iter = line.split(',').map(|x| x.parse::<i64>().unwrap());
        let point = (iter.next().unwrap(), iter.next().unwrap());
        ret.push(point);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_taking_input() {
        let test_example = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

        let p1_inp = parse_input(test_example);
        let ans = part_1(p1_inp);
        assert_eq!(ans, 50);
    }
}
