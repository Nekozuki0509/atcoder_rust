#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
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
        a: [usize;n]
    }

    let mut l = vec![!0; n + 1];
    for i in a {
        let pos = l.lower_bound(&i);
        l[pos] = i;
    }

    println!("{}", l.lower_bound(&!0))
}
