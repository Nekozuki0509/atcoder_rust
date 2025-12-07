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
        a: [usize;n-1],
        b: [usize;n-2]
    }

    let mut dp = vec![1 << 60; n];
    dp[0] = 0;
    for i in 0..n - 1 {
        dp[i + 1] = dp[i + 1].min(dp[i] + a[i]);
        if i + 2 < n {
            dp[i + 2] = dp[i + 2].min(dp[i] + b[i]);
        }
    }

    println!("{}", dp[n - 1]);
}
