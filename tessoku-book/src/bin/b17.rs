#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [isize;n]
    }

    let mut dp = vec![0, (h[0] - h[1]).abs()];
    for (i, &v) in h.iter().enumerate().skip(2) {
        dp.push((dp[i - 1] + (h[i - 1] - h[i]).abs()).min(dp[i - 2] + (h[i - 2] - h[i]).abs()));
    }

    let mut ans = vec![n - 1];
    let mut now = n - 1;
    while now > 0 {
        if dp[now] == dp[now - 1] + (h[now] - h[now - 1]).abs() {
            now -= 1;
        } else {
            now -= 2;
        }

        ans.push(now);
    }

    println!(
        "{}\n{}",
        ans.len(),
        ans.iter().rev().map(|x| (x + 1).to_string()).join(" ")
    );
}
