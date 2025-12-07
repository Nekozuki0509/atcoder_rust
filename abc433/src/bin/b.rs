#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }

    for (i, &v) in a.iter().enumerate() {
        // let mut now = !0usize;
        // let mut an = 0;

        // for (j, &w) in a[..i].iter().enumerate() {
        //     if w > v && now > w - v {
        //         now = w - v;
        //         an = j+1;
        //     }
        // }

        println!("{}", a[..i].iter().enumerate().rev().find(|&(j, &w)| w > v).map(|(j, w)| (j+1) as isize).unwrap_or(-1));
    }
}
