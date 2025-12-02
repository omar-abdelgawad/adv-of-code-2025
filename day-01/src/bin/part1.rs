use std::fs::read_to_string;

const MOD: i64 = 100;
fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let part1_inp = get_part_1_input(filename);
    let ans = part_1(part1_inp);
    dbg!(ans);
}

enum Direction {
    Right(i64),
    Left(i64),
}

fn part_1(directions: Vec<Direction>) -> i64 {
    let mut ans = 0;
    let mut start: i64 = 50;
    for direction in directions {
        match direction {
            Direction::Right(r) => start = (start + r) % MOD,
            Direction::Left(r) => {
                start = ((start + MOD) - r) % MOD;
                assert!(r < MOD, "{r}");
            }
        }
        if start == 0 {
            ans += 1;
        }
    }
    ans
}

fn get_part_1_input(filename: &str) -> Vec<Direction> {
    let inp = read_to_string(filename).unwrap();
    parse_input(&inp)
}

fn parse_input(inp: &str) -> Vec<Direction> {
    let mut ret = Vec::new();
    for line in inp.lines() {
        let rotations: i64 = line[1..].parse::<i64>().unwrap() % MOD;
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
        // too lazy to continue
        let p1_inp = parse_input(test_example);
        let ans = part_1(p1_inp);
        assert_eq!(ans, 3);
    }
}
