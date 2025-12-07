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
        n: usize,
        p: [(f64, f64);n]
    }

    let mut dp = vec![vec![(1usize << 30) as f64; n]; (1 << n) - 1];
    let mut firsts = vec![vec![0; n]; (1 << n) - 1];
    for ((i, &(ix, iy)), (j, &(jx, jy))) in p.iter().enumerate().tuple_combinations() {
        let d = ((ix - jx).powi(2) + (iy - jy).powi(2)).sqrt();
        dp[1 << i][j] = d;
        firsts[1 << i][j] = i;
        dp[1 << j][i] = d;
        firsts[1 << j][i] = j;
    }

    let full = (0..n).collect::<BTreeSet<usize>>();
    for i in (0..n)
        .powerset()
        .filter(|x| x.len() > 1)
        .map(|x| x.iter().copied().collect::<BTreeSet<usize>>())
    {
        let now = i.iter().copied().fold(0, |acc, x| acc | 1 << x);
        for (&k, &j) in iproduct!(full.difference(&i), &i) {
            let nt = dp[now ^ (1 << j)][j] + dp[1 << j][k];
            if dp[now][k] > nt {
                dp[now][k] = nt;
                firsts[now][k] = firsts[now ^ (1 << j)][j];
            }
        }
    }

    let mut min = (1usize << 30) as f64;
    for i in full {
        let now = ((1 << n) - 1) ^ (1 << i);
        min = min.min(dp[now][i] + dp[1 << firsts[now][i]][i]);
    }

    println!("{}", min);
    /*eprintln!(
        "{}",
        dp.iter()
            .map(|x| x.iter().map(|x| format!("{:14.3}", x)).join(" "))
            .join("\n")
    );*/
}
