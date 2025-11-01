#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use itertools::Itertools;
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        (n, m, c): (usize, usize, isize),
        a: [usize;n]
    }

    let mut map = BTreeMap::new();
    for i in a {
        *map.entry(i).or_insert(0usize) += 1;
    }

    let keys = map.keys().copied().collect_vec();
    let mut ans = 0;
    let mut now = 0;
    let mut flag = true;
    let mut first = 0;
    let mut i = 0;
    loop {
        if now >= c {
            if flag {
                if keys.len() == 1 {
                    ans += now * m as isize;
                } else {
                    ans += now * (m - keys.last().unwrap()) as isize;
                }
                flag = false;
            } else {
                ans += now * (keys.get(first+1).unwrap_or(&(keys[first] + 1)) - keys[first]) as isize;
            }
            now = 0.max(now - *map.get(&keys[first]).unwrap() as isize);
            first += 1;

            if first == keys.len() {
                break;
            }

            continue;
        }
        now += *map.get(&keys[i]).unwrap() as isize;
        i += 1;
        i %= map.len();
    }

    println!("{}", ans);
}
