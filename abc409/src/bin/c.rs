use std::collections::BTreeMap;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, l): (usize, usize),
        d: [usize;n-1]
    }

    if l % 3 != 0 {
        println!("0");
        return;
    }

    let mut map = BTreeMap::new();
    *map.entry(0).or_insert(0) += 1usize;

    let mut now = 0;
    for i in d {
        now += i;
        now %= l;
        *map.entry(now).or_insert(0) += 1;
    }

    let mut an = 0;
    let one = l/3;
    for i in 0..one {
        an += *map.entry(i).or_insert(0) * 
                *map.entry(i+one).or_insert(0) *
                *map.entry(i+2*one).or_insert(0)
    }

    //dbg!(map);

    println!("{}", an);
}
