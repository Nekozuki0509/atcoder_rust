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
        mut a: [usize;n]
    }

    let mut v = vec![0];
    for (i, &e) in a.iter().enumerate() {
        v.push(e + v[i]);
    }

    let mut ans = 0;
    let mut now = 1;
    for (i, &e) in v.iter().enumerate() {
        while now <= n && v[now] <= e + k {
            now += 1;
        }

        ans += now - i - 1;
    }

    println!("{ans}");
}
