#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use itertools::Itertools;
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        (n, d): (usize, usize),
        mut a: [usize;n]
    }

    a.sort();
    let mut q = VecDeque::new();
    let mut now = 0;
    for (&f, &s) in a.iter().tuple_windows() {
        if f + d <= s {
            if let Some((x, v)) = cnt[now].1.pop_back() {
                if x == f {
                    cnt[now].1.push_back((x, v+1));
                } else {
                    cnt[now].1.push_back((x, v));
                    cnt[now].1.push_back((f, 2));
                }
            }
        } else {
            now += 1;
            cnt.push((0, VecDeque::new()));
        }
    }

    let mut ans = 0;
    for (i, q) in cnt {}
}
