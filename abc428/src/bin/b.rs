use std::collections::BTreeMap;

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
        s: Chars
    }

    let mut m = BTreeMap::new();
    for i in s.windows(k) {
        //dbg!(i);
        *m.entry(i.iter().collect::<String>()).or_insert(0) += 1;
    }

    let mut b = 0;
    let mut v = vec![];
    for (i, &c) in m.iter() {
        if b < c {
            b = c;
            v = vec![i.clone()];
        } else if b == c {
            v.push(i.clone());
        }
    }

    v.sort();
    
    println!("{}\n{}", b, v.join(" "));
}
