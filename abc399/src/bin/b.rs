

use std::collections::BTreeMap;

#[allow(unused_imports)]
use proconio::{input, marker::{Bytes, Chars, Usize1}};

//#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        p: [usize;n]
    }

    let mut x = BTreeMap::new();
    for i in &p {
        *x.entry(i).or_insert(0) += 1;
    }

    // let mut temp = 0;
    // for i in x.clone().iter().rev() {
    //     if temp == 0 {
    //         temp = *i.1;
    //         continue;
    //     }

    //     *x.entry(i.0).or_insert(0) += temp;
    //     temp = *i.1;
    // }
    
    p.iter().for_each(|i| println!("{}", x.iter().fold(1, |acc, a| {
        if a.0 > &i {
            return acc + a.1;
        }
        acc
    })));
}
