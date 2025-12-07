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
        n: usize,
        a: [usize;n]
    }

    let mut map = BTreeMap::new();
    for (i, &v) in a.iter().enumerate() {
        map.entry(v).or_insert(vec![]).push(i);
    }

    let mut ans = vec![String::new(); n];
    for (i, (_, l)) in map.iter().enumerate() {
        for &j in l {
            ans[j] = (i + 1).to_string();
        }
    }

    println!("{}", ans.join(" "));
}
