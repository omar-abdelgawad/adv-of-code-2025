#![allow(warnings)]

use std::collections::HashMap;

use day_10::*;
/*
* I found a good solution for this problem at:
* https://old.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
* I kind of understand it now after reading the final proof
* The embarrassing part is that my solutino in rust takes about 3 seconds while his python solutin
* takes less than half a second
*/

fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let inp = get_input(filename);
    let ans = part_2(inp);
    dbg!(ans);
}

fn part_2(machine_manuals: Vec<MachineManual>) -> u64 {
    let mut ans = vec![];
    for (i, (_lights, button_effects, joltage_reqs)) in machine_manuals.into_iter().enumerate() {
        //println!("{_lights:?},{button_effects:?},{joltage_reqs:?}");
        let local_ans = fewest_presses(joltage_reqs, button_effects);
        println!("Line {}: answer {local_ans}", i + 1);
        ans.push(local_ans);
        //println!("{}", joltage_reqs.iter().max().unwrap());
    }
    dbg!(&ans);
    ans.iter().sum()
}

type Cache = HashMap<Vec<JoltageRequirement>, u64>;
fn fewest_presses(
    mut joltage_reqs: Vec<JoltageRequirement>,
    button_effects: Vec<ButtonEffect>,
) -> u64 {
    let (joltage_reqs_num, mut button_effects_nums) = get_binary_rep(&joltage_reqs, button_effects);
    let mut cache: Cache = HashMap::new();
    fewest_presses_helper(joltage_reqs, &button_effects_nums, &mut cache)
}

const MAX_BUTTONS: usize = 20;
fn fewest_presses_helper(
    mut joltage_reqs: Vec<JoltageRequirement>,
    button_effects_nums: &[u64],
    cache: &mut Cache,
) -> u64 {
    if is_all_zeroes(&joltage_reqs) {
        return 0;
    } else if let Some(cached_value) = cache.get(&joltage_reqs) {
        return *cached_value;
    }
    let mut fewest_presses = u64::MAX;
    let joltage_reqs_num = get_binary_rep_joltage(&joltage_reqs);
    let num_of_sets = 1 << (button_effects_nums.len());
    for set_num in 0..num_of_sets as u64 {
        let mut joltage_req_clone = joltage_reqs.clone();
        let mut tot_set_xor = 0;
        for element_bit_ind in 0..MAX_BUTTONS {
            if ((1 << element_bit_ind) & set_num) != 0 {
                tot_set_xor ^= button_effects_nums[element_bit_ind];
                if !reduce_joltage(&mut joltage_req_clone, button_effects_nums[element_bit_ind]) {
                    break;
                }
            }
        }
        if tot_set_xor == joltage_reqs_num {
            for joltage in &mut joltage_req_clone {
                *joltage /= 2;
            }
            let next_iter = fewest_presses_helper(joltage_req_clone, button_effects_nums, cache);
            if next_iter != u64::MAX {
                fewest_presses = fewest_presses.min(set_num.count_ones() as u64 + 2 * next_iter)
            }
        }
    }
    //dbg!(&joltage_reqs, fewest_presses);
    cache.insert(joltage_reqs, fewest_presses);
    fewest_presses
}

#[inline(always)]
fn reduce_joltage(joltage_reqs: &mut [JoltageRequirement], bit_encoded_u64: u64) -> bool {
    for element_bit_ind in 0..64 {
        if ((1 << element_bit_ind) & bit_encoded_u64) != 0 {
            if joltage_reqs[element_bit_ind] == 0 {
                return false;
            }
            joltage_reqs[element_bit_ind] -= 1;
        }
    }
    true
}

#[inline(always)]
fn is_all_zeroes(joltage_reqs: &[JoltageRequirement]) -> bool {
    for joltage in joltage_reqs {
        if *joltage != 0 {
            return false;
        }
    }
    true
}

fn get_binary_rep_joltage(joltage_reqs: &[JoltageRequirement]) -> u64 {
    // convert joltages to a u64.
    let mut joltage_req_num = 0u64;
    for (i, joltage) in joltage_reqs.into_iter().enumerate() {
        if joltage % 2 == 1 {
            joltage_req_num |= 1 << i;
        }
    }
    joltage_req_num
}

fn get_binary_rep(
    joltage_reqs: &[JoltageRequirement],
    button_effects: Vec<ButtonEffect>,
) -> (u64, Vec<u64>) {
    let mut joltage_req_num = get_binary_rep_joltage(&joltage_reqs);

    let mut button_effects_nums: Vec<u64> = Vec::new();
    for button_effect in button_effects {
        let mut button_effect_num = 0u64;
        for machine in button_effect {
            button_effect_num ^= 1 << machine;
        }
        button_effects_nums.push(button_effect_num);
    }
    (joltage_req_num, button_effects_nums)
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

        let inp = parse_input(test_example);
        let ans = part_2(inp);
        assert_eq!(ans, 10 + 12 + 11);
    }
    #[test]
    fn test_wrong_case() {
        let test_example = "\
[#..##.] (0,2,4) (0,1,2,3,4) (3,5) (0,1,3) (0) (0,1,4,5) (1,3,4) (1,2) {61,52,36,36,39,28}
";

        let inp = parse_input(test_example);
        let ans = part_2(inp);
        assert_eq!(ans, 90);
    }
}
