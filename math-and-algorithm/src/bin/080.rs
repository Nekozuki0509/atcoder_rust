use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize)
    }

    let mut g = vec![vec![];n];
    for _ in 0..m {
        input! {a: Usize1, b: Usize1, c: usize}
        g[a].push((b, c));
        g[b].push((a, c));
    }

    let mut distance = vec![!0;n];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0));
    for i in 0..n {
        heap.push((Reverse(!0), i));
    }

    while let Some((Reverse(d), target)) = heap.pop() {
        if distance[target] < d {
            continue;
        }

        distance[target] = d;
        for &(next, cost) in &g[target] {
            if distance[next] > d + cost {
                distance[next] = d + cost;
                heap.push((Reverse(distance[next]), next));
            }
        }
    }
    
    println!("{}", if distance[n-1] == !0 {"-1".to_string()} else {distance[n-1].to_string()});
}
