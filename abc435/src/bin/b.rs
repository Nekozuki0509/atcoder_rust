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

    let mut ans = 0;
    for i in 0..n-1 {
        'label: for j in i+1..n {
            let sum = a[i..=j].iter().sum::<usize>();
            for l in a[i..=j].iter() {
                if sum % l == 0 {
                    continue 'label;
                }
            }

            ans += 1;
        }
    }

    println!("{}", ans);
}
