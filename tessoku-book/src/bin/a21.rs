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
        n: usize,
    }

    let mut p = vec![0];
    let mut a = vec![0];
    for _ in 0..n {
        input! {pv: usize, av: usize}
        p.push(pv);
        a.push(av);
    }

    p.push(0);
    a.push(0);

    let mut dp = vec![vec![0; n + 2]; n + 2];
    for i in (0..n - 1).rev() {
        for l in 1..=n - i {
            let r = l + i;

            let score1 = if l <= p[l - 1] && p[l - 1] <= r {
                a[l - 1]
            } else {
                0
            };
            let score2 = if l <= p[r + 1] && p[r + 1] <= r {
                a[r + 1]
            } else {
                0
            };

            dp[l][r] = if l == 1 {
                dp[l][r + 1] + score2
            } else if r == n {
                dp[l - 1][r] + score1
            } else {
                (dp[l - 1][r] + score1).max(dp[l][r + 1] + score2)
            };
        }
    }

    let mut ans = 0;
    for i in 1..=n {
        ans = ans.max(dp[i][i]);
    }

    println!("{}", ans);
}
