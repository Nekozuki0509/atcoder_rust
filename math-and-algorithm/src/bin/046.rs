use std::collections::VecDeque;

use num::abs;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (r, _): (usize, usize),
        s: (usize, usize),
        g: (usize, usize),
        m: [Chars; r]
    }

    let mut set = vec![];
    for (i, r) in m.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if c.eq(&'.') {
                set.push((i + 1, j + 1));
            }            
        }
    }

    let mut d = vec![vec![]; set.len()];
    for (i, v) in set.iter().enumerate() {
        d[i].append(&mut set.iter().enumerate()
                            .filter(|(i, x)| abs(x.0 as isize - v.0 as isize) + abs(x.1 as isize - v.1 as isize) == 1)
                            .map(|(i, x)| i)
                            .collect::<Vec<usize>>()
                        );
    }

    let mut dist = vec![-1; d.len()];

    let mut q = VecDeque::new();
    let first = set.iter().position(|&x| x.eq(&s)).unwrap();
    q.push_back(first);
    dist[first] = 0;

    while !q.is_empty() {
        let pos = q.pop_front().unwrap();

        for i in &d[pos] {
            if dist[*i] == -1 {
                dist[*i] = dist[pos] + 1;
                q.push_back(*i);
            }
        }
    }
    
    // println!("{:?}", set);
    // println!("{:?}", d);
    // println!("{:?}", dist);
    println!("{}", dist[set.iter().position(|&x| x.eq(&g)).unwrap()]);
}
