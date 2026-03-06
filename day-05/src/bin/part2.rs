#![allow(warnings)]

use std::{cmp::max, fs::read_to_string};

fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let part1_inp = get_part_1_input(filename);
    let ans = part_1(part1_inp);
    dbg!(ans);
}

fn part_1((id_ranges, avail_ids): (Vec<(i64, i64)>, Vec<i64>)) -> i64 {
    let mut ans = 0;
    let new_ranges = combine_ranges(id_ranges);
    for range in new_ranges {
        ans += (range.0..=range.1).count() as i64;
    }
    ans
}

fn combine_ranges(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut ret = Vec::new();
    ranges.sort();
    let mut current_range = ranges[0];
    for range in ranges {
        if range.0 <= current_range.1 {
            current_range.1 = max(range.1, current_range.1);
        } else {
            ret.push(current_range);
            current_range = range;
        }
    }
    if &current_range != ret.last().unwrap() {
        ret.push(current_range);
    }
    ret
}

fn get_part_1_input(filename: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let inp = read_to_string(filename).unwrap();
    parse_input(&inp)
}

fn parse_input(inp: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let (ranges_str, ids_str) = inp.split_once("\n\n").unwrap();
    let id_ranges = ranges_str
        .trim()
        .split("\n")
        .map(|range| {
            let (first, second) = range.split_once("-").unwrap();
            let (first_num, second_num) = (first.parse().unwrap(), second.parse().unwrap());
            (first_num, second_num)
        })
        .collect();
    let avail_ids = ids_str
        .trim()
        .split("\n")
        .map(|id_str| id_str.parse().unwrap())
        .collect();
    //dbg!(&id_ranges, &avail_ids);
    (id_ranges, avail_ids)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_taking_input() {
        let test_example = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        let p1_inp = parse_input(test_example);
        let ans = part_1(p1_inp);
        assert_eq!(ans, 14);
    }
}
