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

    let mut dp = vec![vec![isize::MIN; w + 1]; n + 1];
    dp[0][0] = 0;
    for ((i, &(x, y)), j) in iproduct!(d.iter().enumerate(), 0..=w) {
        if j < x {
            dp[i + 1][j] = dp[i][j];
        } else {
            dp[i + 1][j] = dp[i][j].max(dp[i][j - x] + y as isize);
        }
    }

    let mut max = 0;
    for &v in dp[n].iter() {
        max = max.max(v);
    }
    println!("{}", max);
}
