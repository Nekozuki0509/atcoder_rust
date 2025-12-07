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
        a: [usize;n],
    }

    let mut ans = 0;
    let mut now = 0;
    for (i, &v) in a.iter().enumerate() {
        while now + 1 < n && a[now + 1] <= a[i] + k {
            now += 1;
        }

        ans += now - i;
    }

    println!("{ans}");
}
