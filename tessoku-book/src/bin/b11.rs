#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize;n],
        q: usize,
        x: [usize;q]
    }

    a.sort();
    let x = x
        .iter()
        .enumerate()
        .sorted_by_key(|&(_, x)| x)
        .collect_vec();
    let mut ans = vec![String::new(); q];
    let mut now = 0;
    for (i, x) in x {
        let a = a[now..].lower_bound(x);
        ans[i] = (now + a).to_string();
        now = a;
    }

    println!("{}", ans.join("\n"));
}
