use std::collections::VecDeque;

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars
    }
    
    let mut aneed = VecDeque::new();
    let mut bneed = VecDeque::new();
    let mut an = 0usize;
    let mut t = 0;
    for (i, v) in s.iter().enumerate() {
        match t {
            0 => {
                if 'B'.eq(v) {
                    if let Some(x) = bneed.pop_front() {
                        an += i - x;
                    } else {
                        aneed.push_back(i);
                    }
                }
            },
            _ => {
                if 'A'.eq(v) {
                    if let Some(x) = aneed.pop_front() {
                        an += i - x;
                    } else {
                        bneed.push_back(i);
                    }
                }
            }
        }
        t ^= 1;
    }

    let mut an2 = 0;
    let mut t = 1;
    for (i, v) in s.iter().enumerate() {
        match t {
            0 => {
                if 'B'.eq(v) {
                    if let Some(x) = bneed.pop_front() {
                        an2 += i - x;
                    } else {
                        aneed.push_back(i);
                    }
                }
            },
            _ => {
                if 'A'.eq(v) {
                    if let Some(x) = aneed.pop_front() {
                        an2 += i - x;
                    } else {
                        bneed.push_back(i);
                    }
                }
            }
        }
        t ^= 1;
    }

    println!("{}", an.min(an2));
}
