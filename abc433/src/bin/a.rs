#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        (x, y, z): (f64, f64, f64)
    }

    let diff = (z * y - x) / (1. - z);
    let an = if diff == diff.floor() && diff as isize >= 0 {
        "Yes"
    } else {
        "No"
    };

    println!("{}", an);
}
