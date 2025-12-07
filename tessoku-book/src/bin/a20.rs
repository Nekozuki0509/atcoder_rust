#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use itertools::{iproduct, Itertools};
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        s: Bytes,
        t: Bytes
    }

    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    for ((i, &v), (j, &w)) in iproduct!(s.iter().enumerate(), t.iter().enumerate()) {
        dp[i + 1][j + 1] = dp[i + 1][j].max(dp[i][j + 1]);
        if v == w {
            dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j] + 1);
        }
    }

    println!("{}", dp[s.len()][t.len()]);
    eprintln!(
        "{}",
        dp.iter()
            .map(|x| x.iter().map(|x| x.to_string()).join(" "))
            .join("\n")
    );
}
