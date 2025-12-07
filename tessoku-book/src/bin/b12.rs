#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: f64,
    }

    let mut l = 0.;
    let mut r = 100.;
    while l < r {
        let m: f64 = (l + r) / 2.;
        let a = m.powi(3) + m;
        if a < n {
            l = m + 1e-3;
        } else if a > n {
            r = m - 1e-3;
        } else {
            println!("{m}");
            return;
        }
    }

    println!("{l}");
}
