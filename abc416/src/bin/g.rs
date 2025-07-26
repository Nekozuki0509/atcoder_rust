use std::collections::BTreeMap;

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, k, mut x): (usize, usize, isize),
        s: [String;n]
    }

    let mut map = BTreeMap::new();
    for i in &s {
        *map.entry(i).or_insert(0) += 1;
    }
    
    let mut an = String::new();
    for i in 1..=k {
        let one = (k-1..=n-i).fold(1usize, |last, x| last * x);
        dbg!(one);
        for (&j, v) in &map {
            if x - (one * v) as isize <= 0 {
                an.push_str(&j);
                if v == &1 {
                    map.remove(&j);
                } else {
                    *map.entry(&j).or_insert(0) -= 1;
                }
                break;
            } else {
                x -= (one * v) as isize;
            }
        }
    }

    println!("{}", an);

}
