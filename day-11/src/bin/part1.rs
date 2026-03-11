#![allow(warnings)]

use std::collections::HashMap;

use day_11::*;

fn main() {
    let filename = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");
    dbg!(filename);
    let inp = get_input(filename);
    let ans = part_1(inp);
    dbg!(ans);
}

type EdgeList = HashMap<String, Vec<String>>;
type Cache = HashMap<String, u64>;

fn part_1(edges: Vec<Edge>) -> u64 {
    let mut ans = 0;
    let mut edge_list: EdgeList = HashMap::new();
    for (parent, children) in edges {
        match edge_list.insert(parent, children) {
            Some(..) => panic!(),
            None => {}
        }
    }
    let mut cache: Cache = HashMap::new();
    ans = dfs("you", &edge_list, &mut cache);
    ans
}

// assumes no cycles in the tree
// after running turns out cache isn't even needed (actually slows down by 0.1 seconds)
fn dfs(cur_node: &str, edge_list: &EdgeList, cache: &mut Cache) -> u64 {
    if let Some(cached_value) = cache.get(cur_node) {
        return *cached_value;
    } else if cur_node == "out" {
        return 1;
    }
    let mut ret = 0;
    if let Some(children) = edge_list.get(cur_node) {
        for child in children {
            ret += dfs(&child, edge_list, cache)
        }
    }
    cache.insert(cur_node.to_string(), ret);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_taking_input() {
        let test_example = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

        let inp = parse_input(test_example);
        let ans = part_1(inp);
        assert_eq!(ans, 5);
    }
}
