use std::collections::VecDeque;

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
        t: [(usize, usize, usize);n]
    }

    println!("{}", t[0].0);
    let mut que = VecDeque::new();
    que.push_back((t[0].0, t[0].0 + t[0].1, t[0].2));
    for &(s, f, n) in t[1..].iter() {
        let mut now = que.pop_front();
        while now.is_some() && now.unwrap().1 < s {
            now = que.pop_front();
        }
        if now.is_none() {
            que.push_back((s, s+f, n));
            println!("{}", s);
        } else {
            let now = now.unwrap();
            if now.2 + n <= k {
                let mut temp = VecDeque::new();
                temp.push_back(now);
                let next = que.pop_front();
                while next.is_some() && next.unwrap().1 < s+f {
                    temp.push_back(next.unwrap());
                }

                if next.is_none() {
                    while let Some(mut i) = que.pop_back() {
                        i.2 += n;
                        que.push_front(i);
                    }
                } else {
                    let next = next.unwrap();
                    let nextv = next.2;
                    que.push_front(next);
                    que.push_front((s, s+f, nextv + n));
                    while let Some(mut i) = temp.pop_back() {
                        i.2 += n;
                        que.push_front(i);
                    }
                }
            } else {
                
            }
        }
    }
}
