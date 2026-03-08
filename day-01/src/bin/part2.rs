use std::fs::read_to_string;

const MOD: i64 = 100;
static mut ANS: i64 = 0;
fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let part1_inp = get_part_2_input(filename);
    let ans = part_2(part1_inp);
    dbg!(ans);
}

enum Direction {
    Right(i64),
    Left(i64),
}

fn part_2(directions: Vec<Direction>) -> i64 {
    let mut start: i64 = 50;
    for direction in directions {
        let old_start = start;
        match direction {
            Direction::Right(r) => {
                start = (start + r) % MOD;
                if start < old_start {
                    unsafe {
                        ANS += 1;
                    }
                }
            }
            Direction::Left(r) => {
                start = ((start + MOD) - r) % MOD;
                if (start > old_start && old_start != 0) || start == 0 {
                    unsafe {
                        ANS += 1;
                    }
                }
                //assert!(r < MOD, "{r}");
            }
        }
        eprintln!("here: {start}, {}", unsafe { ANS });
    }
    unsafe { ANS }
}

fn get_part_2_input(filename: &str) -> Vec<Direction> {
    let inp = read_to_string(filename).unwrap();
    parse_input(&inp)
}

fn parse_input(inp: &str) -> Vec<Direction> {
    let mut ret = Vec::new();
    for line in inp.lines() {
        let old_rotations: i64 = line[1..].parse::<i64>().unwrap();
        let rotations: i64 = old_rotations % MOD;
        let num_times_hit_zero = old_rotations - rotations;
        assert!(old_rotations != 0);
        assert!(num_times_hit_zero % MOD == 0);
        unsafe { ANS += num_times_hit_zero / 100 }
        if rotations == 0 {
            continue;
        }
        match line.chars().next().unwrap() {
            'R' => ret.push(Direction::Right(rotations)),
            'L' => ret.push(Direction::Left(rotations)),
            _ => unreachable!(),
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_taking_input() {
        unsafe {
            ANS = 0;
        }
        let test_example = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let p2_inp = parse_input(test_example);
        let ans = part_2(p2_inp);
        assert_eq!(ans, 6);
    }
    // don't run the two test cases at the same time since a race condition might occur
    #[ignore = "just run one of the tests at a time due to static variable"]
    #[test]
    fn test_other_case() {
        unsafe {
            ANS = 0;
        }
        let test_example = "\
R1000
L50";
        let p2_inp = parse_input(test_example);
        let ans = part_2(p2_inp);
        assert_eq!(ans, 11);
    }
}
