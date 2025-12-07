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
        h: [isize;n]
    }

    let mut dp = vec![0, (h[0] - h[1]).abs()];
    for i in 2..n {
        dp.push((dp[i - 2] + (h[i - 2] - h[i]).abs()).min(dp[i - 1] + (h[i - 1] - h[i]).abs()));
    }

    println!("{}", dp[n - 1]);
}
