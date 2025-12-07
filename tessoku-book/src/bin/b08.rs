#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use itertools::iproduct;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [(usize, usize);n],
        q: usize
    }

    let mut v = vec![vec![0; 1501]; 1501];
    for (x, y) in p {
        v[x][y] += 1;
    }

    for (i, j) in iproduct!(1..=1500, 2..=1500) {
        v[i][j] += v[i][j - 1];
    }

    for (i, j) in iproduct!(2..=1500, 1..=1500) {
        v[i][j] += v[i - 1][j];
    }

    for _ in 0..q {
        input! {a: usize, b: usize, c: usize, d: usize}
        println!("{}", v[a - 1][b - 1] + v[c][d] - v[a - 1][d] - v[c][b - 1]);
    }
}
