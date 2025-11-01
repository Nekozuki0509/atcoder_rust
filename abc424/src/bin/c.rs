use std::collections::{BTreeSet, VecDeque};

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut g = vec![vec![];n+1];
    let mut q = VecDeque::new();
    let mut dist = vec![-1; n+1];
    let mut ans = 0;

    for i in 1..=n {
        input! {a: usize, b: usize}
        if a == 0 && b == 0 {
            q.push_back(i);
            dist[i] = 0;
            ans += 1;
            continue;
        }

        g[a].push(i);
        g[b].push(i);
    }

    while !q.is_empty() {
        let pos = q.pop_front().unwrap();

        for i in &g[pos] {
            if dist[*i] == -1 {
                dist[*i] = dist[pos] + 1;
                ans += 1;
                q.push_back(*i);
            }
        }
    }

    println!("{}", ans);
}
