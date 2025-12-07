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
        (n, m): (usize, usize),
        a: [[usize;n];m]
    }

    let a = a
        .into_iter()
        .map(|x| x.iter().enumerate().fold(0, |acc, (i, &v)| acc | v << i))
        .collect_vec();
    let mut dp = vec![vec![1usize << 60; 1 << n]; m + 1];
    dp[0][0] = 0;
    for (i, j) in iproduct!(0..m, 0..(1 << n)) {
        dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);
        dp[i + 1][j | a[i]] = dp[i + 1][j | a[i]].min(dp[i][j] + 1);
    }

    if dp[m][(1 << n) - 1] > m {
        println!("-1");
    } else {
        println!("{}", dp[m][(1 << n) - 1]);
    }
}
