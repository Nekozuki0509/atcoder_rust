use pathfinding::prelude::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex};
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {n: u32, s: Chars}
        let mut g = DiGraph::<usize, usize>::new();
        for i in 0..=s.len() {
            g.add_node(i);
        }
        for i in 0..s.len() {
            for j in 0u32..n {
                g.add_edge(NodeIndex::new(i), NodeIndex::new(i+2usize.pow(j)), 0);
            }
        }
        for (i, v) in s.iter().enumerate() {
            if v.eq(&'1') {
                g.remove_node(NodeIndex::new(i+1));
            }
        }
    }
}
