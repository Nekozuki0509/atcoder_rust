use std::collections::VecDeque;


use num::abs;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (h, w): (usize, usize),
        r: [Chars; h]
    }

    let mut eset = vec![];
    let mut aset = vec![];

    for (i, s) in r.iter().enumerate() {
        for (j, c) in s.iter().enumerate() {
            if c.eq(&'.') {
                aset.push((i + 1, j + 1));
            } else if c.eq(&'E') {
                aset.push((i + 1, j + 1));
                eset.push((i + 1, j + 1));
            }
        }
    }

    let mut d = vec![vec![]; aset.len()];
    for (i, v) in aset.iter().enumerate() {
        d[i].append(&mut aset.iter().enumerate()
                            .filter(|(i, x)| abs(x.0 as isize - v.0 as isize) + abs(x.1 as isize - v.1 as isize) == 1)
                            .map(|(i, x)| i)
                            .collect::<Vec<usize>>()
                        );
    }

    let mut dist = vec![-1; d.len()];

    let mut q = VecDeque::new();
    let epos = eset.iter().map(|&(x, y)| aset.iter().position(|&(ax, ay)| x == ax && y == ay).unwrap()).collect::<Vec<usize>>();
    for first in epos {
        q.push_back(first);
        dist[first] = 0;
    
        while !q.is_empty() {
            let pos = q.pop_front().unwrap();
    
            for i in &d[pos] {
                if dist[*i] == -1 || dist[*i] > dist[pos] + 1 {
                    dist[*i] = dist[pos] + 1;
                    q.push_back(*i);
                }
            }
        }
    }

    let mut now = 0usize;
    for (i, s) in r.iter().enumerate() {
        for (j, c) in s.iter().enumerate() {
            if c.eq(&'.') {
                let mut nd = 2isize.pow(60);
                let mut min: isize = now as isize;
                for v in &d[now] {
                    if dist[*v] < nd {
                        nd = dist[*v];
                        min = *v as isize;
                    }
                }
                let minp = aset[min as usize];
                let nowp = aset[now];
                let minp = (minp.0 as isize, minp.1 as isize);
                let nowp = (nowp.0 as isize, nowp.1 as isize);
                //dbg!(now, minp, nowp);
                let an = if minp.1 - nowp.1 == -1 {
                    "<"
                } else if minp.1 - nowp.1 == 1 {
                    ">"
                } else if minp.0 - nowp.0 == -1 {
                    "^"
                } else {
                    "v"
                };
                print!("{}", an);
                //dbg!(an);

                now += 1;
            } else {
                if c.eq(&'E') {
                    now += 1;
                }

                print!("{}", c);
            }
        }
        println!();
    }
}
