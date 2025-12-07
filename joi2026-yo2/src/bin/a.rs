#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1, Isize1}};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize;n]
    }

    a.sort();

    let now;
    let mut cnt = 0;
    let mut last = 0;
    if n % 2 == 0 {
        now = n / 2 - 1;
        last = a[now];
        loop {
            if last != a[now-cnt] {
                if last != a[now+cnt+1] {
                    println!("{}", a[now+cnt+1]);
                } else {
                    println!("{}", a[now-cnt+1]);
                }
    
                return;
            } else if last != a[now+cnt+1] {
                println!("{}", a[now+cnt+1]);
                return;
            }

            cnt += 1;
        }
    } else {
        now = n / 2;
        if a[now] != a[now+1] {
            println!("{}", a[now+1]);
            return;
        } else if a[now] != a[now-1] {
            println!("{}", a[now]);
            return;
        }

        last = a[now];
        loop {
            if last != a[now-cnt] {
                if last != a[now+cnt] {
                    println!("{}", a[now+cnt]);
                } else {
                    println!("{}", a[now-cnt+1]);
                }
    
                return;
            } else if last != a[now+cnt] {
                println!("{}", a[now+cnt]);
                return;
            }

            cnt += 1;
        }
    }
}
