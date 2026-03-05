//#![allow(warnings)]
use std::fs::read_to_string;

fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let part2_inp = get_part_2_input(filename);
    let ans = part_2(part2_inp);
    dbg!(ans);
}

fn part_2(banks: Vec<Vec<i64>>) -> i64 {
    let mut ans = Vec::new();
    const NUM_DIGITS: i64 = 12;
    for bank in banks {
        let mut joltage = 0;
        let mut start_ind = 0;
        for dig_ind in (0..NUM_DIGITS).rev() {
            let last_ind = bank.len() - dig_ind as usize;
            let highest_earliest_first_dig_ind = bank[start_ind..last_ind]
                .iter()
                .enumerate()
                .max_by_key(|&(index, value)| (value, -(index as i64)))
                .map(|(index, _)| index)
                .unwrap()
                + start_ind; // last line because enumerate starts at 0 not orig index
            let highest_earliest_first_dig = bank[highest_earliest_first_dig_ind];
            joltage += highest_earliest_first_dig * 10i64.pow(dig_ind as u32);
            start_ind = highest_earliest_first_dig_ind + 1;
        }

        ans.push(joltage);
    }
    //dbg!(&ans);
    ans.iter().sum()
}

fn get_part_2_input(filename: &str) -> Vec<Vec<i64>> {
    let inp = read_to_string(filename).unwrap();
    parse_input(&inp)
}

fn parse_input(inp: &str) -> Vec<Vec<i64>> {
    let mut ret = Vec::new();
    for (i, bank) in inp.trim().split("\n").enumerate() {
        ret.push(vec![]);
        for dig in bank.chars() {
            ret[i].push(dig.to_digit(10).unwrap() as i64);
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
987654321111111
811111111111119
234234234234278
818181911112111";

        let p1_inp = parse_input(test_example);
        let ans = part_2(p1_inp);
        assert_eq!(ans, 3121910778619);
    }
}
