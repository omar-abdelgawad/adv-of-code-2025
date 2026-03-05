#![allow(warnings)]

use std::fs::read_to_string;

fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let part1_inp = get_part_1_input(filename);
    let ans = part_1(part1_inp);
    dbg!(ans);
}

fn part_1((id_ranges, avail_ids): (Vec<(i64, i64)>, Vec<i64>)) -> i64 {
    todo!();
    let ans = 0;
    ans
}

fn get_part_1_input(filename: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let inp = read_to_string(filename).unwrap();
    parse_input(&inp)
}

fn parse_input(inp: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    todo!();
    //let mut ret = Vec::new();
    //for (i, line) in inp.trim().split("\n").enumerate() {
    //    ret.push(vec![]);
    //    for dig in line.chars() {
    //        ret[i].push(dig);
    //    }
    //}
    //ret
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
        assert_eq!(ans, 3);
    }
}
