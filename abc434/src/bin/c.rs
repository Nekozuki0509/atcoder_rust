#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    'outer: for _ in 0..t {
        input! {
            (n, h): (usize, isize),
            d: [(isize, isize, isize);n]
        }

        let mut now = (0, h, h);
        for (t, l, u) in d {
            if ((l <= now.1 && u >= now.1) || (l >= now.1 && l <= now.2)) || t - now.0 >= (now.2 - l).abs().min((now.1 - u).abs()) {
                now = (t, l.max(now.1-t+now.0), u.min(now.2+t-now.0));
            } else {
                //dbg!(now, t, l, u);
                println!("No");
                continue 'outer;
            }
        }

        println!("Yes");
    }
}
