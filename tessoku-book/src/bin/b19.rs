#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use itertools::iproduct;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    ops::Index,
};

#[fastout]
fn main() {
    input! {
        (n, w): (usize, usize),
        d: [(usize, usize);n]
    }

    let sum = d.iter().fold(0, |acc, &(_, v)| acc + v);
    let mut dp = vec![vec![!0; sum + 1]; n + 1];
    dp[0][0] = 0;
    for ((i, &(x, y)), j) in iproduct!(d.iter().enumerate(), 0..=sum) {
        if j < y {
            dp[i + 1][j] = dp[i][j];
        } else if let Some(x) = x.checked_add(dp[i][j - y]) {
            dp[i + 1][j] = dp[i][j].min(x);
        } else {
            dp[i + 1][j] = dp[i][j];
        }
    }

    println!(
        "{}",
        dp[n]
            .iter()
            .copied()
            .enumerate()
            .rev()
            .find(|&(_, v)| v <= w)
            .unwrap()
            .0
    );
}
