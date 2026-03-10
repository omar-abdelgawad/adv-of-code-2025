#![allow(warnings)]

use std::fs::read_to_string;

fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let inp = get_part_1_input(filename);
    let ans = part_1(inp);
    dbg!(ans);
}

type Machine = u64;
type Joltage = u64;
type ButtonEffect = Vec<Machine>;
type JoltageRequirement = Joltage;
type MachineManual = (Vec<bool>, Vec<ButtonEffect>, Vec<JoltageRequirement>);

fn part_1(machine_manuals: Vec<MachineManual>) -> u64 {
    let mut ans = 0;
    for (lights, button_effects, _joltage_reqs) in machine_manuals {
        println!("{lights:?},{button_effects:?},{_joltage_reqs:?}");
        ans += fewest_presses(lights, button_effects);
    }
    ans
}

fn fewest_presses(lights: Vec<bool>, button_effects: Vec<ButtonEffect>) -> u64 {
    // solution: just try all combinations since they are small.
    let (lights_num, button_effects_nums) = get_binary_rep(lights, button_effects);
    //println!("{lights_num:#b}");
    //for v in &button_effects_nums {
    //    print!("{v:#b} ");
    //}
    //println!();

    let mut fewest = u64::MAX;
    let num_of_sets = 1 << (button_effects_nums.len());
    for set_num in 0..num_of_sets as u64 {
        let mut tot_set_xor = 0;
        for element_bit_ind in 0..64 {
            if ((1 << element_bit_ind) & set_num) != 0 {
                tot_set_xor ^= button_effects_nums[element_bit_ind];
            }
        }
        if tot_set_xor == lights_num {
            fewest = fewest.min(set_num.count_ones() as u64);
        }
    }
    fewest
}

fn get_binary_rep(lights: Vec<bool>, button_effects: Vec<ButtonEffect>) -> (u64, Vec<u64>) {
    // convert lights to a u64.
    let mut lights_num = 0u64;
    for (i, light) in lights.into_iter().enumerate() {
        if light {
            lights_num |= 1 << i;
        }
    }
    let mut button_effects_nums: Vec<u64> = Vec::new();
    for button_effect in button_effects {
        let mut button_effect_num = 0u64;
        for machine in button_effect {
            button_effect_num ^= 1 << machine;
        }
        button_effects_nums.push(button_effect_num);
    }
    (lights_num, button_effects_nums)
}

fn get_part_1_input(filename: &str) -> Vec<MachineManual> {
    let inp = read_to_string(filename).unwrap();
    parse_input(&inp)
}

fn parse_input(inp: &str) -> Vec<MachineManual> {
    inp.trim().lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> MachineManual {
    let mut parts = line.split_whitespace();
    let lights = parts
        .next()
        .unwrap()
        .trim_matches(['[', ']'])
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<bool>>();
    let joltage_requirements: Vec<JoltageRequirement> = parts
        .next_back()
        .unwrap()
        .trim_matches(['{', '}'])
        .split(',')
        .map(|x| x.parse::<Joltage>().unwrap())
        .collect();
    let mut button_effects = Vec::new();
    for p in parts {
        let nums = p
            .trim_matches(['(', ')'])
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        button_effects.push(nums);
    }
    (lights, button_effects, joltage_requirements)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_taking_input() {
        let test_example = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

        let p1_inp = parse_input(test_example);
        let ans = part_1(p1_inp);
        assert_eq!(ans, 2 + 3 + 2);
    }
}
