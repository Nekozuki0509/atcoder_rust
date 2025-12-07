#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use itertools::iproduct;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
        a: [usize;n],
        b: [usize;n],
        c: [usize;n],
        d: [usize;n]
    }

    let mut p = vec![];
    for (i, j) in iproduct!(a, b) {
        p.push(i + j);
    }

    let mut q = vec![];
    for (i, j) in iproduct!(c, d) {
        q.push(i + j);
    }
    q.sort();

    for i in p {
        if q.binary_search(&(k - i)).is_ok() {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
