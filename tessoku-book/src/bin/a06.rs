#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        a: [usize;n]
    }

    let mut v = vec![0];
    for (i, &n) in a.iter().enumerate() {
        v.push(v[i] + n);
    }

    for _ in 0..q {
        input! {l: usize, r: usize}
        println!("{}", v[r] - v[l - 1]);
    }
}
