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
        a: [Usize1;n-1],
        b: [Usize1;n-1]
    }

    let mut dp = vec![isize::MIN; n];
    dp[0] = 0;
    for i in 0..n - 1 {
        dp[a[i]] = dp[a[i]].max(dp[i] + 100);
        dp[b[i]] = dp[b[i]].max(dp[i] + 150);
    }

    println!("{}", dp[n - 1]);
}
