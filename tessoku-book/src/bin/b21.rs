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
        (n, s): (usize, Chars)
    }

    let mut dp = vec![vec![0; n]; n];
    for i in 0..n {
        dp[i][i] = 1;
    }
    for i in 0..n - 1 {
        dp[i][i + 1] = if s[i] == s[i + 1] { 2 } else { 1 };
    }

    for i in 2..n {
        for l in 0..n - i {
            let r = l + i;
            dp[l][r] = dp[l + 1][r].max(dp[l][r - 1]);
            if s[l] == s[r] {
                dp[l][r] = dp[l][r].max(dp[l + 1][r - 1] + 2);
            }
        }
    }

    println!("{}", dp[0][n - 1]);
}
