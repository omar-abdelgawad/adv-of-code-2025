#![allow(warnings)]

use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let part1_inp = get_part_1_input(filename);
    let ans = part_1(part1_inp);
    dbg!(ans);
}

// could be u64 but then I have to carefully handle subtraction
type Point = (i64, i64, i64);
type PIndex = usize;

fn part_1(points: Vec<Point>) -> i64 {
    let num_points = points.len();
    let mut distances = get_distances(&points);
    distances.sort();
    let mut junction_points: Vec<Vec<PIndex>> = vec![vec![]; num_points];
    let mut map: HashMap<PIndex, usize> = HashMap::new(); // value is circuit index
    for i in 0..num_points {
        junction_points[i].push(i);
        map.insert(i, i);
    }
    for i in 0..distances.len() {
        let (p1i, p2i) = distances[i].1;
        if map[&p1i] == map[&p2i] {
            continue; // in same circuit
        }
        let mut junction_point_remove = map[&p2i];
        let mut junction_point_increase = map[&p1i];
        if junction_point_remove < junction_point_increase {
            std::mem::swap(&mut junction_point_remove, &mut junction_point_increase);
        }
        for i in 0..junction_points[junction_point_remove].len() {
            let pindex = junction_points[junction_point_remove][i];
            junction_points[junction_point_increase].push(pindex);
            *map.get_mut(&pindex).unwrap() = junction_point_increase;
        }
        junction_points[junction_point_remove].clear();
        if junction_points[0].len() == num_points {
            return points[p1i].0 * points[p2i].0;
        }
    }
    panic!();
}

fn get_distances(points: &Vec<Point>) -> Vec<(i64, (PIndex, PIndex))> {
    let mut ret = vec![];
    for i in 0..points.len() - 1 {
        for j in i + 1..points.len() {
            ret.push((dist_squared(&points[i], &points[j]), (i, j)));
        }
    }
    ret
}

fn dist_squared(point: &Point, other: &Point) -> i64 {
    // if you want u64 you can use point.0.abs_diff(other.0)
    let dx = point.0 - other.0;
    let dy = point.1 - other.1;
    let dz = point.2 - other.2;
    (dx * dx + dy * dy + dz * dz)
}

fn get_part_1_input(filename: &str) -> Vec<Point> {
    let inp = read_to_string(filename).unwrap();
    parse_input(&inp)
}

fn parse_input(inp: &str) -> Vec<Point> {
    let mut ret = vec![];
    for line in inp.trim().lines() {
        let mut iter = line.split(',').map(|x| x.parse::<i64>().unwrap());
        let point = (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        );
        ret.push(point);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_taking_input() {
        let test_example = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

        let p1_inp = parse_input(test_example);
        let ans = part_1(p1_inp);
        assert_eq!(ans, 25272);
    }
}
