use std::collections::VecDeque;

use itertools::Itertools;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: isize,
        a: [isize;n]
    }

    let mut vec = vec![false;(n+1) as usize];
    for i in &a {
        if *i != -1 && !vec[*i as usize] {
            vec[*i as usize] = true;
        } else if *i == -1 {
        } else {
            println!("No");
            return;
        }
    }

    let mut ans = vec![];
    let mut now = 1;
    for (i, &v) in a.iter().enumerate() {
        if v == -1 {
            while vec[now] {
                now += 1;
            }

            ans.push(now);
            now += 1;
        } else {
            ans.push(v as usize);
        }
    }

    println!("Yes");
    println!("{}", ans.iter().map(|x| x.to_string()).join(" "));
}
