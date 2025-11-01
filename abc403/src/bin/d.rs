use std::collections::BTreeMap;

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        mut a: [usize; n]
    }

    let mut map = BTreeMap::new();
    for i in a {
        *map.entry(i).or_insert(0) += 1;
    }
    
    for i in 0..d {
        
    }
    
    println!("{}", an);
}
