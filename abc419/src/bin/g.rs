use std::collections::{BTreeSet, VecDeque};

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
    }

    let mut g = vec![vec![];n+1];
    for _ in 0..m {
        input! {u: usize, w: usize}
        g[u].push(w);
        g[w].push(u);
    }

    let mut dist = vec![BTreeSet::new();n+1];

    let mut q = VecDeque::new();
    q.push_back(1);
    dist[1].insert(0);

    while !q.is_empty() {
        let pos = q.pop_front().unwrap();

        if pos == n {
            break;
        }

        for i in &g[pos] {
            if dist[*i].is_empty() {
                dist[*i] = dist[pos].iter().map(|&x| x+1).collect();
                q.push_back(*i);
            } else {
                let mut dists = dist[pos].clone().into_iter().map(|x| x+1).collect();
                dist[*i].append(&mut dists);
                if !q.contains(i) {
                    q.push_back(*i);
                }
            }
        }
    }
}
