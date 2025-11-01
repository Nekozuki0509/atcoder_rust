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
        (n, k): (usize, usize)
    }

    let mut ans = 0;
    for i in 1..=n {
        for j in 1..=n {
            if i + j < k && k - i - j <= n {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
