use std::collections::VecDeque;

use num::abs;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (h, w): (usize, usize),
        m: [Chars;h]
    }

    let mut start = 0;
    let mut goal = 0;
    let mut rset = vec![];
    let mut oset = vec![];
    let mut xset = vec![];
    let mut bset = vec![];
    for (i, r) in m.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            match c {
                '.' => rset.push((i + 1, j + 1)),
                'S' => {
                    start = rset.len();
                    rset.push((i + 1, j + 1));
                },
                'G' => {
                    goal = rset.len();
                    rset.push((i + 1, j + 1));
                },
                'o' => {
                    oset.push(rset.len());
                    rset.push((i + 1, j + 1));
                },
                'x' => {
                    xset.push(rset.len());
                    rset.push((i + 1, j + 1));
                },
                '?' => {
                    bset.push(rset.len());
                    rset.push((i + 1, j + 1));
                },
                _ => {}
            }  
        }
    }

    let mut d = vec![vec![]; rset.len()];
    for (i, v) in rset.iter().enumerate() {
        d[i].append(&mut rset.iter().enumerate()
                            .filter(|(i, x)| abs(x.0 as isize - v.0 as isize) + abs(x.1 as isize - v.1 as isize) == 1)
                            .map(|(i, x)| i)
                            .collect::<Vec<usize>>()
                        );
    }

    let mut g = 2usize.pow(60);
    let mut dist = vec![(false, false); d.len()];
    dist[start] = (true, false);

    let mut q: VecDeque<(usize, usize, bool)> = VecDeque::new();
    q.push_back((0, start, true));

    while !q.is_empty() {
        let (now, pos, t) = q.pop_front().unwrap();
        dbg!(now, pos, t);

        for i in &d[pos] {
            if (t && xset.contains(i)) || (!t && oset.contains(i)) || (dist[*i].0 && t) || (dist[*i].1 && !t) {
                //dbg!(i, t, pos);
                continue;
            }
            if *i == goal {
                g = g.min(now);
                continue;
            }
            if t {
                dist[pos].0 = true;
            } else {
                dist[pos].1 = true;
            }
            if bset.contains(i) {
                q.push_back((now + 1, *i, !t));
            } else {
                q.push_back((now + 1, *i, t));
            }

            //dbg!(&q);
        }
    }

    if g == 2usize.pow(60) {
        println!("-1");
    } else {
        println!("{}", g);
    }

    //dbg!(start, goal, oset, xset, bset, !0);
}
