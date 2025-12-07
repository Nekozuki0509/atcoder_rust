#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use itertools::Itertools;
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        a: [isize;n]
    }

    let max = *a.iter().max().unwrap();
    for _ in 0..q {
        input! {c: isize, d: isize}
        let mut l = 0;
        let mut r = max / d + 1;
        loop {
            let m = (l+r) / 2;
            let mut sum1 = c*m;
            let mut sum2 = c*(m+1);
            for &i in &a {
                sum1 += 0.max(i-m*d);
                sum2 += 0.max(i-(m+1)*d);
            }

            if l + 1 >= r {
                let mut sum1 = c*l;
                let mut sum2 = c*r;
                for &i in &a {
                    sum1 += 0.max(i-l*d);
                    sum2 += 0.max(i-r*d);
                }

                println!("{}", sum1.min(sum2));
                break;
            }

            if sum1 < sum2 {
                r = m;
            } else if sum1 > sum2 {
                l = m+1;
            } else {
                println!("{}", sum1);
                break;
            }
        }
    }
}
