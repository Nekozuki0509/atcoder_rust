#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        (a, b, c, d): (usize, usize, usize, usize)
    }

    if c < a {
        println!("No");
    } else if c >= a && d < b {
        println!("Yes");
    } else {
        println!("No");
    }
}
