#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [(usize, isize, isize);n]
    }

    let sum = p.iter().map(|&(w, _, _)| w).sum::<usize>();
    let mut dp = vec![vec![-1;sum/2+1];n+1];
    dp[0][0] = 0;
    for (i, &(w, h, b)) in p.iter().enumerate() {
        for j in 0..=sum/2 {
            if dp[i][j] != -1 {
                if j + w <= sum / 2 {
                    dp[i+1][j+w] = dp[i+1][j+w].max(dp[i][j] + h);
                }
                dp[i+1][j] = dp[i+1][j].max(dp[i][j] + b);
            }
        }
    }

    let mut an = 0;
    for i in 0..=sum/2 {
        an = an.max(dp[n][i]);
    }
    
    println!("{}", an);
}
