use std::fs::read_to_string;

fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let part2_inp = get_part_2_input(filename);
    let ans = part_2(part2_inp);
    dbg!(ans);
}

fn part_2(ranges: Vec<(i64, i64)>) -> i64 {
    let mut ans = 0;
    for (left, right) in ranges {
        for possible_invalid_num in left..=right {
            let possible_invalid_num_str = possible_invalid_num.to_string().into_bytes();
            if is_invalid_id(&possible_invalid_num_str) {
                eprintln!("{possible_invalid_num}");
                ans += possible_invalid_num;
            }
        }
    }
    ans
}
fn is_invalid_id(invalid_id: &[u8]) -> bool {
    'outer: for len_repeated_segment in 1..=(invalid_id.len() / 2) {
        if !invalid_id.len().is_multiple_of(len_repeated_segment) {
            continue;
        }
        for i in 0..len_repeated_segment {
            let mut j = 1;
            while i + len_repeated_segment * j < invalid_id.len() {
                if invalid_id[i] != invalid_id[i + len_repeated_segment * j] {
                    continue 'outer;
                }
                j += 1;
            }
        }
        return true;
    }
    false
}

fn get_part_2_input(filename: &str) -> Vec<(i64, i64)> {
    let inp = read_to_string(filename).unwrap();
    parse_input(&inp)
}

fn parse_input(inp: &str) -> Vec<(i64, i64)> {
    let mut ret = Vec::new();
    for range in inp.trim().split(",") {
        let (left, right) = range.split_once('-').unwrap();
        let range: (i64, i64) = (left.parse().unwrap(), right.parse().unwrap());
        ret.push(range);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_taking_input() {
        let test_example = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124";

        let p1_inp = parse_input(test_example);
        let ans = part_2(p1_inp);
        assert_eq!(ans, 4174379265);
    }
}
