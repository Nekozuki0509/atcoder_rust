use std::{cmp::min, collections::BTreeSet};

use num::abs;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize)
    }

    let mut g = vec![vec![];n+1];
    for _ in 0..m {
        input! {a: usize, b: usize}
        g[a].push(b);
        g[b].push(a);
    }

    //let alla = 0usize;
    let all = abs(m as isize - n as isize);

    // let mut la = 0isize;
    // let mut aa = 0isize;
    let mut da = 0isize;
    let mut stack = BTreeSet::new();
    for (v, i) in g[1..].iter().enumerate() {
        //let mut an = (i.len() as isize) - 2;
        // if an > 0 {
        //     aa += an;
        // } else if an < 0 {
        //     la -= an;
        // }
        let l = (i.len() as isize) - 2;
        if l < 0 {
            for j in 1..=n {
                if j == v {
                    continue;
                }
            }
        } else {
            da += l;
        }
    }
    dbg!(&all, &da);
    dbg!(&stack);
    println!("{}", all + da);
}
