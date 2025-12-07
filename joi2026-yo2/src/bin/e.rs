#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize)
    }

    let mut g = vec![vec![];n];
    for _ in 0..m {
        input! {a: Usize1, b: Usize1}
        g[a].push(b);
        g[b].push(a);
    }
    
    for i in 0..n {
        let mut visited = vec![false;n];
        let mut cnt = 0;
        dfs(i, &mut visited, &g, &mut cnt, true);
        println!("{}", n - cnt);
    }

}

fn dfs(pos: usize, visited: &mut Vec<bool>, g: &Vec<Vec<usize>>, cnt: &mut usize, flag: bool) {
    *cnt += 1;
    visited[pos] = true;
    for &i in &g[pos] {
        if !visited[i] && ((flag && i < pos) || (flag && i > pos)) {
            dfs(i, visited, g, cnt, !flag);
        }
    }
}
