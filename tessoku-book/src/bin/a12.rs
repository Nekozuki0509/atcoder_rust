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
        (n, k): (usize, usize),
        a: [usize;n]
    }

    let mut l = 0usize;
    let mut r = 1e9 as usize;
    while l < r {
        let m = (l + r) / 2;
        let mut sum = 0;
        for &i in &a {
            sum += m / i;
        }

        if sum < k {
            l = m + 1;
        } else {
            r = m;
        }
    }

    println!("{l}");
}
