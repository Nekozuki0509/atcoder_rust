#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::{collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque}, mem::swap};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize)
    }

    let mut g = vec![vec![];n];
    for _ in 0..m {
        input! {x: Usize1, y: Usize1}
        g[y].push(x);
    }

    input! {q: usize}
    let mut blacks = vec![false;n];
    for _ in 0..q {
        input! {t: u8, v: Usize1}
        match t {
            1 => {
                let mut q = VecDeque::new();
                q.push_back(v);

                while let Some(next) = q.pop_front() {
                    if !blacks[next] {
                        blacks[next] = true;
                        for &i in &g[next] {
                            if !blacks[i] {
                                q.push_back(i);
                            }
                        }
                    }
                }
            },
            _ => println!("{}", if blacks[v] {"Yes"} else {"No"})
        }
    }
}