use std::collections::{BTreeMap, BTreeSet};

#[allow(unused_imports)]
use proconio::{input, marker::{Bytes, Chars, Usize1}};

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars
    }
    let mut map = BTreeMap::new();
    let mut set = BTreeSet::new();
    for i in 0..n {
        if !(*map.entry(s[i]).or_insert(t[i])).eq(&t[i]) {
            println!("-1");
            return;
        }
    }

    let mut an = 0;
    for i in &map {
        if i.0.eq(&i.1) || set.contains(&(i.0, i.1)) {
            continue;
        }

        if map.keys().filter(|&x| x.eq(i.0)).count() > 1 {
            an += 2;
        } else {
            an += 1;
        }

        set.insert((i.0, i.1));
    }

    
    println!("{}", an);
}
