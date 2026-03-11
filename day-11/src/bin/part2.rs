#![allow(warnings)]

use day_11::*;
use std::collections::HashMap;

fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let inp = get_input(filename);
    let ans = part_2(inp);
    dbg!(ans);
}

type EdgeList = HashMap<String, Vec<String>>;
type Cache = HashMap<String, (u64, u64, u64, u64)>;

fn part_2(edges: Vec<Edge>) -> u64 {
    let mut ans = 0;
    let mut edge_list: EdgeList = HashMap::new();
    for (parent, children) in edges {
        match edge_list.insert(parent, children) {
            Some(..) => panic!(),
            None => {}
        }
    }
    let mut cache: Cache = HashMap::new();
    ans = dfs("svr", &edge_list, &mut cache).3;
    ans
}
// assumes no cycles in the tree
// returns (paths seeing none, paths seeing only fft paths seeing only dac, paths seeing both fft and dac)
fn dfs(cur_node: &str, edge_list: &EdgeList, cache: &mut Cache) -> (u64, u64, u64, u64) {
    if let Some(cached_value) = cache.get(cur_node) {
        return *cached_value;
    } else if cur_node == "out" {
        return (1, 0, 0, 0);
    }
    let mut ret = (0, 0, 0, 0);
    if let Some(children) = edge_list.get(cur_node) {
        for child in children {
            let child_ret_value = dfs(&child, edge_list, cache);
            ret.0 += child_ret_value.0;
            ret.1 += child_ret_value.1;
            ret.2 += child_ret_value.2;
            ret.3 += child_ret_value.3;
        }
    }

    if cur_node == "fft" || cur_node == "dac" {
        let paths_seeing_none = 0;
        let mut paths_seeing_only_fft;
        let mut paths_seeing_only_dac;
        let mut paths_seeing_both;
        if cur_node == "dac" {
            paths_seeing_only_fft = 0;
            paths_seeing_only_dac = ret.0 + ret.2;
            paths_seeing_both = ret.1 + ret.3;
        } else {
            paths_seeing_only_dac = 0;
            paths_seeing_only_fft = ret.0 + ret.1;
            paths_seeing_both = ret.2 + ret.3;
        }
        ret = (
            paths_seeing_none,
            paths_seeing_only_fft,
            paths_seeing_only_dac,
            paths_seeing_both,
        )
    }
    cache.insert(cur_node.to_string(), ret);
    //dbg!(cur_node, ret);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_taking_input() {
        let test_example = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

        let inp = parse_input(test_example);
        let ans = part_2(inp);
        assert_eq!(ans, 2);
    }
}
