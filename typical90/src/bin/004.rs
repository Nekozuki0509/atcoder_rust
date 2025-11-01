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
        (h, w): (usize, usize),
        a: [[usize;w];h]
    }

    let mut hv: Vec<usize> = vec![0; h];
    let mut wv: Vec<usize> = vec![0; w];

    for (i, v) in a.iter().enumerate() {
        for (j, &z) in v.iter().enumerate() {
            hv[i] += z;
            wv[j] += z;
        }
    }

    for (i, v) in a.iter().enumerate() {
        for (j, &z) in v.iter().enumerate() {
            print!("{} ", hv[i] + wv[j] - z);
        }
        println!()
    }
}
