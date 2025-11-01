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
        q: usize
    }

    let mut v = vec![0];
    for (i, &n) in a.iter().enumerate() {
        v.push(v[i] + n);
    }

    for _ in 0..q {
        input! {l: usize, r: usize}
        if v[r] - v[l - 1] < r + 1 + v[l - 1] - l - v[r] {
            println!("lose");
        } else if v[r] - v[l - 1] == r + 1 + v[l - 1] - l - v[r] {
            println!("draw");
        } else {
            println!("win");
        }
    }
}
