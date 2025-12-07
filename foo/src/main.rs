#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use itertools::Itertools;
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n],
        b: [usize;n]
    }

    let mut map = BTreeMap::new();
    let mut qa = VecDeque::new();
    let mut qb = VecDeque::new();
    
    let mut last = 0;
    for (i, &v) in a.iter().enumerate() {
        *map.entry(v).or_insert(0) += 1;
        if last < v {
            if let Some((lv, _)) = qa.pop_back() {
                qa.push_back((lv, i-1));
            }
            qa.push_back((v, i));
            last = v;
        }
    }
    
    let mut last = 0;
    for (i, &v) in b.iter().enumerate() {
        if i != 0 {
            *map.entry(v).or_insert(0) += 1;
        }

        if last < v {
            if let Some((lv, _)) = qb.pop_back() {
                qa.push_back((lv, i-1));
            }
            qb.push_back((v, i));
            last = v;
        }
    }

    let (mut na, mut nai) = qa.pop_front().unwrap();
    let mut nal = 0;
    let (mut nb, mut nbi) = qb.pop_front().unwrap();
    let mut nbl = 0;
    loop {
        if na > nb {
            *map[&na] += (nai as isize - nal as isize).abs() * (nbi as isize - nbl as isize).abs();
            
        }
    }

    println!("{}", now.iter().join("\n"));
}
