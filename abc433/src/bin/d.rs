#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use itertools::Itertools;
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        a: [usize;n]
    }

    let mut set: BTreeMap<usize, BTreeMap<usize, usize>> = BTreeMap::new();
    for i in a {
        *set.entry(i%m).or_insert(BTreeMap::new()).entry(i.to_string().len()).or_insert(0usize) += 1;
    }

    //dbg!(&set);

    let mut ans = 0;
    for (&i, v) in &set {
        let mut l = v.iter().fold(0, |acc, (_, &v)| acc + v);
        for j in 1..=10 {
            let n = (i.to_string() + &"0".repeat(j)).parse::<usize>().unwrap();
            let nm = (m - n % m) % m;
            //dbg!(n, nm, j, set.contains_key(&nm));
            if set.contains_key(&nm) {
                //dbg!(&set[&nm]);
                if let Some(&x) = set[&nm].get(&j) {
                    //dbg!(v.len(), x);
                    ans += l * x;
                }
            }
        }
    }

    println!("{}", ans);
}
