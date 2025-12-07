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
        (h, w): (usize, usize),
        x: [[usize;w];h],
        q: usize
    }

    let mut nx = vec![vec![0; w + 1]; h + 1];

    for (i, j) in iproduct!(0..h, 0..w) {
        nx[i + 1][j + 1] = nx[i + 1][j] + x[i][j];
    }

    for (i, j) in iproduct!(0..h, 0..w) {
        nx[i + 1][j + 1] += nx[i][j + 1];
    }

    for _ in 0..q {
        input! {a: usize, b: usize, c: usize, d: usize}
        println!(
            "{}",
            nx[a - 1][b - 1] + nx[c][d] - nx[a - 1][d] - nx[c][b - 1]
        );
    }
}
