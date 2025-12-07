#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        (w, b): (usize, usize)
    }

    let an = (w * 1000 + 1) / b;
    if (w * 1000 + 1) % b == 0 {
        println!("{}", an);
    } else {
        println!("{}", an + 1);
    }
}
