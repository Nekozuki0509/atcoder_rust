#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        a: [usize;n]
    }

    let s = a.iter().sum::<usize>();

    for i in a {
        if s - i == m {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
