use std::collections::{BTreeMap, VecDeque};

use itertools::Itertools;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize)
    }

    let mut g = vec![vec![];n+1];
    for _ in 0..m {
        input! {v: usize, w: usize}
        g[v].push(w);
        g[w].push(v);
    }

    let mut dist = vec![(-1, vec![]); n + 1];

    let mut q = VecDeque::new();
    q.push_back(1);
    dist[1] = (0, vec![1]);

    let mut ans = 0;
    while !q.is_empty() {
        let mut now = 0;
        let mut map: BTreeMap<(usize, usize), Vec<usize>> = BTreeMap::new();
        while !q.is_empty() {
            let pos = q.pop_front().unwrap();
    
            for &i in &g[pos] {
                if dist[i].0 == -1 {
                    let mut tmp = vec![];
                    tmp.append(&mut dist[pos].1.clone());
                    tmp.push(i);

                    dist[i] = (dist[pos].0 + 1, tmp);
                    q.push_back(i);
                } else if dist[pos].0 % 2 == dist[i].0 % 2 {
                    map.entry((pos.min(i), pos.max(i))).or_insert(vec![]).push(now);
                    let mut last = i;
                    for &i in dist[i].1.iter().rev() {
                        if let Some(x) = dist[pos].1.iter().position(|&x| x == i) {
                            for &i in dist[pos].1[x..].iter() {
                                map.entry((last.min(i), last.max(i))).or_insert(vec![]).push(now);
                                last = i;
                            }
                            now += 1;
                            break;
                        } else {
                            map.entry((last.min(i), last.max(i))).or_insert(vec![]).push(now);
                            last = i;
                        }
                    }
                }
            }

            let mut f = vec![false;now];
            dbg!(&map);
            let nv = map.iter().sorted_by_key(|(_, v)| v.len()).map(|(_, v)| v.clone()).rev().collect::<Vec<Vec<usize>>>();
            for (v, i) in nv.iter().enumerate() {
                let mut flag = false;
                for &j in i {
                    if !f[j] {
                        if !flag {
                            ans += 1;
                            dbg!(v);
                            flag = true;
                        }

                        f[j] = true;
                    }
                }
            }
        }

        //ans += set.len();

        for (i, v) in dist.clone().iter().enumerate() {
            if i == 0 {
                continue;
            }

            if v.0 == -1 {
                q.push_back(i);
                dist[i] = (0, vec![i]);
            }
        }
    }

    println!("{}", ans);
}
