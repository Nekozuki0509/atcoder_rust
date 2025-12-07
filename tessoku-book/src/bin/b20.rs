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
        s: Bytes,
        t: Bytes
    }

    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    for i in 0..=t.len() {
        dp[0][i] = i;
    }
    for i in 0..=s.len() {
        dp[i][0] = i;
    }

    for ((i, &v), (j, &w)) in iproduct!(s.iter().enumerate(), t.iter().enumerate()) {
        let d = if v == w { 0 } else { 1 };
        dp[i + 1][j + 1] = (dp[i][j + 1] + 1).min(dp[i + 1][j] + 1).min(dp[i][j] + d);
    }

    println!("{}", dp[s.len()][t.len()]);
}
