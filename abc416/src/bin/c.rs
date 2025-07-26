use std::collections::BTreeMap;

use itertools::Itertools;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, k, x): (usize, usize, usize),
        s: [String;n]
    }

    let mut ans = vec![];
    let mut buf = vec![String::new();k];

    dfs(&mut buf, 0, n, &s, &mut ans);

    ans.sort();

    println!("{}", ans[x-1]);

}

fn dfs(buf: &mut Vec<String>, d:usize, ub: usize, s: &Vec<String>, ans: &mut Vec<String>) {
    if d == buf.len() {
        ans.push(buf.concat());
    } else {
        for i in 0..ub {
            buf[d] = s[i].clone();
            dfs(buf, d+1, ub, s, ans);
        }        
    }
}
