#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use itertools::Itertools;
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        d: [(Usize1, usize);q]
    }

    let mut v = BTreeMap::new();
    for (i, &(l, r)) in d.iter().enumerate() {
        v.entry(l).or_insert(vec![]).push((1usize, i));
        v.entry(r).or_insert(vec![]).push((0, i));
    }

    let mut ans = BTreeMap::new();
    let mut now = vec![0;n];
    for (j, i) in v {
        for (j, v) in i {
            now[v] = j;
        }

        let mut flag = true;
        for (i, &v) in now.iter().enumerate() {
            if flag && v == 1 {
                *ans.entry(i).or_insert(0) += v;
                flag = false;
            }
        }
    }

    let mut nv = vec![ans[&0];q];
    for i in 1..q {
        nv[i] = nv[i-1] + ans.get(&i).unwrap_or(&0);
    }

    dbg!(&nv);

    println!("{}", nv.iter().map(|x| (n-x).to_string()).join("\n"));
}
