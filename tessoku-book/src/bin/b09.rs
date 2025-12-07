#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use itertools::{iproduct, Itertools};
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut v = vec![vec![0isize; 1501]; 1501];
    for _ in 0..n {
        input! {a: usize, b: usize, c: usize, d: usize}
        v[a][b] += 1;
        v[c][d] += 1;
        v[a][d] -= 1;
        v[c][b] -= 1;
    }

    for (i, j) in iproduct!(0..=1500, 1..=1500) {
        v[i][j] += v[i][j - 1];
    }

    for (i, j) in iproduct!(1..=1500, 0..=1500) {
        v[i][j] += v[i - 1][j];
    }

    let mut ans = 0usize;
    for (i, j) in iproduct!(0..=1500, 0..=1500) {
        if v[i][j] > 0 {
            ans += 1;
        }
    }

    println!("{ans}");
}
