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
        a: [usize;n-1],
        b: [usize;n-2]
    }

    let mut dp = vec![0, a[0]];
    for i in 2..n {
        dp.push((dp[i - 1] + a[i - 1]).min(dp[i - 2] + b[i - 2]));
    }

    let mut ans = vec![];
    let mut now = n - 1;
    loop {
        ans.push((now + 1).to_string());
        if now == 0 {
            break;
        }

        if dp[now] == dp[now - 1] + a[now - 1] {
            now -= 1;
        } else {
            now -= 2;
        }
    }

    println!("{}\n{}", ans.len(), ans.iter().rev().join(" "));
}
