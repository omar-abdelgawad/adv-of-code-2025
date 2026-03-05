use std::fs::read_to_string;

fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let part1_inp = get_part_1_input(filename);
    let ans = part_1(part1_inp);
    dbg!(ans);
}

fn part_1(banks: Vec<Vec<i64>>) -> i64 {
    let mut ans = 0;
    for bank in banks {
        let last_ind = bank.len() - 1;
        let highest_earliest_first_dig_ind = bank[0..last_ind]
            .iter()
            .enumerate()
            .max_by_key(|&(index, value)| (value, -(index as i64)))
            .map(|(index, _)| index)
            .unwrap();
        let highest_earliest_first_dig = bank[highest_earliest_first_dig_ind];
        let mut joltage = highest_earliest_first_dig * 10;
        let second_dig = bank[highest_earliest_first_dig_ind + 1..]
            .iter()
            .max()
            .unwrap();
        joltage += second_dig;
        ans += joltage;
    }
    ans
}

fn get_part_1_input(filename: &str) -> Vec<Vec<i64>> {
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
        let ans = part_1(p1_inp);
        assert_eq!(ans, 357);
    }
}
