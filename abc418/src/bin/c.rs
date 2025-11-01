use std::collections::BTreeMap;

use itertools::Itertools;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, qn): (usize, usize),
        mut a: [usize;n],
        q: [usize;qn]
    }

    a.sort();
    let max = a[n-1];

    let q = q
        .iter().enumerate()
        .sorted_by_key(|&(_, v)| v)
        .collect::<Vec<_>>();
    
    let mut an = 0;
    let mut ans = vec![];
    let mut i = 0;
    for (j, &b) in q {
        if b > max {
            ans.push((j, -1));
            continue;
        }

        while a[i] < b {
            an += a[i];
            i += 1;
        }
        ans.push((j, (an+(b-1)*(n-i)+1) as isize));
        //dbg!(j, b, an+(b-1)*(n-i)+1);
    }

    println!("{}", ans.iter().sorted_by_key(|(i, _)| i).map(|(_, v)| v.to_string()).join("\n"))
}
