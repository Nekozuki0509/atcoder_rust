#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        (mut x, n): (usize, usize),
        w: [usize;n],
        q: usize,
        p: [Usize1;q]
    }

    let mut v = vec![false;n];
    for i in p {
        if v[i] {
            x -= w[i];
            v[i] = false;
        } else {
            x += w[i];
            v[i] = true;
        }

        println!("{}", x);
    }
}
