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
        (d, n): (usize, usize)
    }

    let mut v = vec![0isize; d + 1];
    for _ in 0..n {
        input! {l: Usize1, r: Usize1}
        v[l] += 1;
        v[r + 1] -= 1;
    }

    let mut now = 0;
    for (i, &v) in v[..d].iter().enumerate() {
        now += v;
        println!("{}", now);
    }
}
