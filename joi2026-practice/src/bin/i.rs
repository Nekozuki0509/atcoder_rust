use std::collections::VecDeque;

use itertools::Itertools;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize
    }

    let mut que = VecDeque::new();
    for _ in 0..n {
        input! {a: usize}
        que.push_back(a);
    }

    input! {
        q: usize,
        x: [usize;q]
    }

    let mut an = vec![];
    let mut last = 0;
    let mut now = 0usize;
    for i in x {
        while now < i {
            let mut l = que.pop_front().unwrap();
            let mut cnt = 1usize;
            while l % 2 == 0 {
                cnt *= 2;
                l /= 2;
            }
    
            now += cnt;
            last = l;
        }

        an.push(last.to_string());
    }

    println!("{}", an.join("\n"))
}
