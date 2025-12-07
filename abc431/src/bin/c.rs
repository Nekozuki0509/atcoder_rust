#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        (n, m, k): (usize, usize, usize),
        mut h: [usize;n],
        mut b: [usize;m]
    }

    h.sort();
    h.reverse();
    b.sort();
    b.reverse();
    
    let mut cnt = 0;
    let mut now_h = 0;
    let mut now_b = 0;
    while cnt < k {
        if now_h == n || now_b == m {
            println!("No");
            return;
        }

        if h[now_h] > b[now_b] {
            now_h += 1;
        } else {
            cnt += 1;
            now_h += 1;
            now_b += 1;
        }
    }

    println!("Yes");
}
