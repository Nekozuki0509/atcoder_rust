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
        (h, w, n) : (usize, usize, usize),
    }

    let mut v = vec![vec![0isize; w + 1]; h + 1];
    for _ in 0..n {
        input! {a: Usize1, b: Usize1, c: Usize1, d: Usize1}
        v[a][b] += 1;
        v[c + 1][b] -= 1;
        v[a][d + 1] -= 1;
        v[c + 1][d + 1] += 1;
    }

    for (i, j) in iproduct!(0..h, 1..w) {
        v[i][j] += v[i][j - 1];
    }

    for (i, j) in iproduct!(1..h, 0..w) {
        v[i][j] += v[i - 1][j];
    }

    println!(
        "{}",
        v[..h].iter().map(|x| x[..w].iter().join(" ")).join("\n")
    );
}
