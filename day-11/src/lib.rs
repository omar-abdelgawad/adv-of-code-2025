use std::fs::read_to_string;

pub type Machine = u64;
pub type Joltage = u64;
pub type ButtonEffect = Vec<Machine>;
pub type JoltageRequirement = Joltage;
pub type MachineManual = (Vec<bool>, Vec<ButtonEffect>, Vec<JoltageRequirement>);

pub type Edge = (String, Vec<String>);

pub fn get_input(filename: &str) -> Vec<Edge> {
    let inp = read_to_string(filename).unwrap();
    parse_input(&inp)
}

pub fn parse_input(inp: &str) -> Vec<Edge> {
    inp.trim().lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Edge {
    let parts = line.split_once(": ").unwrap();
    let parent = parts.0;
    let children: Vec<String> = parts.1.split_whitespace().map(|x| x.to_string()).collect();
    (parent.to_string(), children)
}
