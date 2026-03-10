use std::fs::read_to_string;

pub type Machine = u64;
pub type Joltage = u64;
pub type ButtonEffect = Vec<Machine>;
pub type JoltageRequirement = Joltage;
pub type MachineManual = (Vec<bool>, Vec<ButtonEffect>, Vec<JoltageRequirement>);

pub fn get_input(filename: &str) -> Vec<MachineManual> {
    let inp = read_to_string(filename).unwrap();
    parse_input(&inp)
}

pub fn parse_input(inp: &str) -> Vec<MachineManual> {
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
