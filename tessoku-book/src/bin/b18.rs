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
        (n, s): (usize, usize),
        a: [usize;n]
    }

    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    for ((i, &v), j) in iproduct!(a.iter().enumerate(), 0..=s) {
        if dp[i][j] {
            dp[i + 1][j] = true;
            if j + v <= s {
                dp[i + 1][j + v] = true;
            }
        }
    }

    if !dp[n][s] {
        println!("-1");
        return;
    }

    let mut ans = vec![];
    let mut now = s;
    for i in (0..n).rev() {
        if !dp[i][now] {
            ans.push((i + 1).to_string());
            now -= a[i];
        }
    }

    println!("{}\n{}", ans.len(), ans.join(" "));
}
