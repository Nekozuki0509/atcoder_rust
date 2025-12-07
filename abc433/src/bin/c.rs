#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use itertools::Itertools;
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let s = s.iter().map(|&x| x.to_string().parse::<usize>().unwrap()).collect_vec();
    let mut last = (s[0], 0usize, 0, 1usize);
    let mut flag = true;
    let mut ans = 0;
    for i in s {
        if flag {
            if last.0 == i {
                last.1 += 1;
            } else {
                flag = false;
                last.2 = i;
            }

            continue;
        } 
        
        if last.2 == i {
            last.3 += 1;
        } else {
            //dbg!(last);
            if last.0 + 1 == last.2 {
                ans += last.1.min(last.3);
            }

            last.0 = last.2;
            last.1 = last.3;
            last.2 = i;
            last.3 = 1;
        }
    }

    if last.0 + 1 == last.2 {
        ans += last.1.min(last.3);
    }

    println!("{}", ans);
}
