use std::collections::VecDeque;

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        w: [usize;n]
    }

    let mut g = vec![vec![];n+1];
    for _ in 0..m {
        input! {v: usize, w: usize}
        g[v].push(w);
        g[w].push(v);
    }

    let mut dist = vec![!0; n + 1];

    let mut q = VecDeque::new();
    q.push_back((1, w[0]));
    dist[1] = 0;

    while !q.is_empty() {
        let (pos, we) = q.pop_front().unwrap();

        for &i in &g[pos] {
            if dist[i] > dist[pos] + we {
                //dbg!(pos, i, dist[i], dist[pos] * 2, w[pos-1]);
                dist[i] = dist[pos] + we;
                q.push_back((i, we + w[i-1]));
            }
        }
    }

    for i in dist[1..].iter() {
        println!("{}", i);
    }
}
