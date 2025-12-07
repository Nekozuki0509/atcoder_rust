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
        (n, x): (usize, usize),
        a: [usize;n]
    }

    let (mut l, mut r) = (0, n - 1);
    let mut ans = !0;
    while l <= r {
        let m = (l + r) / 2;
        if x < a[m] {
            r = m - 1;
        } else if x == a[m] {
            ans = m;
            break;
        } else {
            l = m + 1;
        }
    }

    println!("{}", ans + 1);
}
