#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: [(Usize1, Usize1, Usize1, Usize1);n]
    }

    let mut v = vec![vec![vec![];2001];2001];
    for (i, &(u, d, l, r)) in d.iter().enumerate() {
        v[u][l].push(i);
        v[d+1][l].push(i);
        v[u][r+1].push(i);
        v[d+1][r+1].push(i);
    }

    let mut clears = 0usize;
    let mut map = BTreeMap::new();
    for i in 0..2000 {
        for j in 1..2000 {
            let mut tmp = v[i][j-1].clone();
            v[i][j].append(&mut tmp);
        }
    }

    for i in 1..2000 {
        for j in 0..2000 {
            let mut tmp = v[i-1][j].clone();
            v[i][j].append(&mut tmp);
        }
    }

    for i in 0..2000 {
        for j in 0..2000 {
            if v[i][j].len() == 0 {
                clears += 1;
            } else if v[i][j].len() == 1 {
                *map.entry(v[i][j][0]).or_insert(0) += 1;
            }
        }
    }

    for i in 0..n {
        println!("{}", clears + map.get(&i).unwrap_or(&0));
    }
}
