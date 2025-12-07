#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use itertools::Itertools;
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: Chars,
    }

    let n = n.iter().sorted().collect_vec();
    let mut cnt = 0;
    let mut flag = true;
    for i in n {
        if flag {
            if '0'.eq(i) {
                cnt += 1;
            } else {
                print!("{}", i);
                print!("{}", 0.to_string().repeat(cnt));
                flag = false;
            }
        } else {
            print!("{}", i);
        }
    }
}
