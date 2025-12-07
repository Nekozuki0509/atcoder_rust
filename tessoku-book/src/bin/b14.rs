#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    iter::once,
};

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
        a: [usize;n]
    }

    let (l, r) = a.split_at(n / 2);

    let set = l
        .iter()
        .powerset()
        .map(|x| x.into_iter().sum::<usize>())
        .sorted()
        .collect_vec();
    let any = r
        .iter()
        .powerset()
        .map(|x| x.into_iter().sum::<usize>())
        .any(|x| set.binary_search(&(k - x)).is_ok());

    println!("{}", if any { "Yes" } else { "No" });
}
