use std::collections::BTreeSet;

use amplify::confinement::Collection;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        c: [usize; n]
    }

    let mut k = vec![vec![]; m + 1];

    let mut set = vec![vec![]; n + 1];
    for i in 1..=m {
        input! {l: usize, mut a: [usize; l]}

        for j in a {
            set[j].push(i);
            k[i].push(j);
        }
    }

    let mut dp = vec![0];
    let mut watched: Vec<usize> = vec![];
    for i in 1..=m {
        let mut min = 1_000_000_001;
        let mut min_m = 0;
        let now = watched.iter().filter(|&&x| x == i).count();
        if now == 1 {
            for j in &k[m] {
                if min > c[j-1] {
                    min = c[j-1];
                    min_m = *j;
                }
            }
            for j in &set[min_m] {
                watched.push(*j);
            }
            dp.push(dp[i-1] + min);
        } else if now == 0 {
            for j in &k[m] {
                if min > c[j-1] {
                    min = c[j-1];
                    min_m = *j;
                }
            }
            for j in &set[min_m] {
                watched.push(*j);
                watched.push(*j);
            }
            dp.push(dp[i-1] + min * 2);
        } else {
            dp.push(dp[i-1]);
        }
    }
    
    println!("{}", dp[m]);
}
