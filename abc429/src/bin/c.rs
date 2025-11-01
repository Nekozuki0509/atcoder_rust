#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }

    let mut map = BTreeMap::new();
    for i in a {
        *map.entry(i).or_insert(0usize) += 1;
    }

    let mut ans = 0;
    for (_, v) in map {
        if v < 2 {
            continue;
        }

        ans += (v * (v - 1) / 2) * (n - v);
    }

    println!("{}", ans);
}
