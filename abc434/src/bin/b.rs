#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        d: [(Usize1, f64);n]
    }

    let mut v = vec![(0, 0.);m];
    for (i, w) in d {
        v[i] = (v[i].0 + 1, v[i].1 + w);
    }

    for (i, w) in v {
        println!("{}", w / i as f64)
    }
}
