#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }

    let mut now = 0;
    let mut ans = 0;
    for (i, &e) in a.iter().enumerate() {
        if i <= now {
            now = now.max(i+e-1);
            ans += 1;
        } else {
            break;
        }
    }


    println!("{ans}");
}
