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
        n: usize,
        a: [usize;n],
        d: usize
    }

    let mut lv = vec![0];
    for (i, &v) in a.iter().enumerate() {
        lv.push(lv[i].max(v));
    }

    let mut rv = vec![0];
    for (i, &v) in a.iter().rev().enumerate() {
        rv.push(rv[i].max(v));
    }
    rv.push(0);
    rv.reverse();

    for _ in 0..d {
        input! {l: usize, r: usize}
        println!("{}", lv[l - 1].max(rv[r + 1]));
    }
}
