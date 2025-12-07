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
        (n, s): (usize, usize),
        a: [usize;n]
    }

    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    for (i, &v) in a.iter().enumerate() {
        for j in 0..=s {
            if dp[i][j] {
                dp[i + 1][j] = true;
                if j + v <= s {
                    dp[i + 1][j + v] = true;
                }
            }
        }
    }

    println!("{}", if dp[n][s] { "Yes" } else { "No" });
}
