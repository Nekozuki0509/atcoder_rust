use itertools::Itertools;
use std::collections::VecDeque;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize)
    }

    let mut g = vec![vec![];n+1];
    for _ in 0..m {
        input! {a: usize, b: usize}
        g[a].push(b);
        g[b].push(a);
    }

    let mut dist = vec![120usize; n + 1];

    let mut q = VecDeque::new();
    q.push_back(1);
    dist[1] = 0;

    while !q.is_empty() {
        let pos = q.pop_front().unwrap();

        for i in &g[pos] {
            if dist[*i] > dist[pos] + 1 {
                dist[*i] = dist[pos] + 1;
                q.push_back(*i);
            }
        }
    }
    
    println!("{}", dist[1..].iter().map(|x| x.to_string()).join("\n"));
}
