#![allow(warnings)]

use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let part1_inp = get_part_2_input(filename);
    let ans = part_2(part1_inp);
    dbg!(ans);
}

// could be u64 but then I have to carefully handle subtraction
type Point = (i64, i64);
type Edge = (Point, Point);

fn part_2(mut points: Vec<Point>) -> i64 {
    let mut ans = 0;
    let num_points = points.len();
    for i in 0..num_points - 1 {
        for j in i + 1..num_points {
            let mut flag = false;
            let rect_area = area(&points[i], &points[j]);
            if rect_area <= ans {
                continue;
            }
            for k in 0..num_points {
                if k == i || k == j {
                    continue;
                }
                // if point is inside the rectangle excluding edges
                if points[k].in_between(&points[i], &points[j]) {
                    //eprintln!("point {k} is between points {i} and {j}");
                    flag = true;
                    break;
                }
                // dont forget to handle case of point on the edge
                let next_ind = (k + 1) % num_points;
                if (points[k], points[next_ind]).in_between(&points[i], &points[j]) {
                    //eprintln!("line formed by {k} and {next_ind} crosses rectangle {i} and {j}");
                    flag = true;
                    break;
                } else {
                    //eprintln!("point {k} is on the edges of {i} and {j} but its fine");
                }
            }
            if flag {
                continue;
            }
            ans = rect_area;
        }
    }
    ans
}
trait Between {
    fn in_between(&self, p1: &Point, p2: &Point) -> bool;
}
trait OnEdge {
    fn on_edge(&self, p1: &Point, p2: &Point) -> bool;
}

impl Between for Edge {
    // check that the edge crosses the rectangle formed by p1 and p2
    fn in_between(&self, p1: &Point, p2: &Point) -> bool {
        let bottom_left = (p1.0.min(p2.0), p1.1.max(p2.1));
        let top_right = (p1.0.max(p2.0), p1.1.min(p2.1));
        let (mut ep1, mut ep2) = *self;
        if (ep1.0 == ep2.0) {
            // vertical Edge
            if ep1.1 < ep2.1 {
                std::mem::swap(&mut ep1, &mut ep2);
            }
            if !(ep1.0 < top_right.0 && ep1.0 > bottom_left.1) {
                return false;
            }
            return (ep1.1 > top_right.1 && ep2.1 <= top_right.1)
                || (ep1.1 >= bottom_left.1 && ep2.1 < bottom_left.1);
        } else if (ep1.1 == ep2.1) {
            // horizontal Edge
            if ep1.0 > ep2.0 {
                std::mem::swap(&mut ep1, &mut ep2);
            }
            if !(ep1.1 > top_right.1 && ep1.1 < bottom_left.1) {
                return false;
            }
            return (ep1.0 < top_right.0 && ep2.0 >= top_right.0)
                || (ep1.0 <= bottom_left.0 && ep2.0 > bottom_left.0);
        } else {
            panic!("edges should only be horizontal or vertical");
        }
    }
}

impl Between for Point {
    fn in_between(&self, p1: &Point, p2: &Point) -> bool {
        let bottom_left = (p1.0.min(p2.0), p1.1.max(p2.1));
        let top_right = (p1.0.max(p2.0), p1.1.min(p2.1));

        (self.0 > bottom_left.0 && self.0 < top_right.0)
            && (self.1 < bottom_left.1 && self.1 > top_right.1)
    }
}
impl OnEdge for Point {
    fn on_edge(&self, p1: &Point, p2: &Point) -> bool {
        let bottom_left = (p1.0.min(p2.0), p1.1.max(p2.1));
        let top_right = (p1.0.max(p2.0), p1.1.min(p2.1));

        let (x, y) = *self;
        let (x1, y1) = bottom_left; // y1 is bottom (larger)
        let (x2, y2) = top_right; // y2 is top (smaller)

        ((x == x1 || x == x2) && y <= y1 && y >= y2) || ((y == y1 || y == y2) && x >= x1 && x <= x2)
    }
}

fn area(p1: &Point, p2: &Point) -> i64 {
    ((p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1)) as i64
}

fn get_part_2_input(filename: &str) -> Vec<Point> {
    let inp = read_to_string(filename).unwrap();
    parse_input(&inp)
}

fn parse_input(inp: &str) -> Vec<Point> {
    let mut ret = vec![];
    for line in inp.trim().lines() {
        let mut iter = line.split(',').map(|x| x.parse::<i64>().unwrap());
        let point = (iter.next().unwrap(), iter.next().unwrap());
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
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

        let p1_inp = parse_input(test_example);
        let ans = part_2(p1_inp);
        assert_eq!(ans, 24);
    }
}
