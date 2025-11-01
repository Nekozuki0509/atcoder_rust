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
        (t, n): (usize, usize)
    }

    let mut v = vec![0isize; t + 1];
    for _ in 0..n {
        input! {l: usize, r: usize}
        v[l] += 1;
        v[r] -= 1;
    }

    let mut now = 0;
    for &i in v[..t].iter() {
        now += i;
        println!("{}", now);
    }
}
