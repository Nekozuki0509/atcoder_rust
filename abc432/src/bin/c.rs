#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        (n, x, y): (usize, usize, usize),
        a: [usize;n]
    }

    let min = *a.iter().min().unwrap();
    let diff = y - x;
    let mut max = min;
    if y % diff != 0 {
        let nd = diff / gcd(y, diff);
        let mut now = 0;
        let mut flag = true;
        for &i in &a {
            if flag {
                now = i % nd;
                flag = false;
                continue;
            }

            if now != i % nd {
                println!("-1");
                return;
            }
        }

        if now != 0 {
            max -= nd - now;
        }
    }

    let z = max * y + (min - max) * x;
    let mut ans = 0;
    for i in a {
        let need = (y * i - z) / diff;
        if need > i {
            println!("-1");
            return;
        }
        ans += i - (y * i - z) / diff;
    }

    println!("{}", ans);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}