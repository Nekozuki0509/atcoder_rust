use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let mut dist = vec![2usize.pow(60); k];
    let mut q = VecDeque::new();

    dist[1] = 1;
    q.push_front(1usize);

    while !q.is_empty() {
        let v = q.pop_front().unwrap();

        let v2 = (v * 10) % k;
        if dist[v2] > dist[v] {
            dist[v2] = dist[v];
            q.push_front(v2);
        }

        let v2 = (v + 1) % k;
        if dist[v2] > dist[v] + 1 {
            dist[v2] = dist[v] + 1;
            q.push_back(v2);
        }
    }
    
    println!("{}", dist[0]);
}
