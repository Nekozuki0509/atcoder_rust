use std::collections::BTreeMap;

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }
    let min = *a.iter().min().unwrap();

    let mut map = BTreeMap::new();
    for i in &a {
        *map.entry(min).or_insert(0) += 1isize;
        *map.entry(i+1).or_insert(0) -= 1;
    }

    //dbg!(&map);

    let mut nmap = BTreeMap::new();
    let mut last = 0;
    for (i, &v) in map.iter() {
        if *i == min {
            last = v;
            nmap.insert(min, last);
            continue;
        }

        nmap.insert(*i-1, last);
        last += v;
    }

    //dbg!(&nmap);

    let mut last = *a.iter().max().unwrap() as isize;
    for (&i, &v) in nmap.iter().rev() {
        if v >= i as isize {
            println!("{}", (i as isize).max(last));
            return;
        }

        last = v as isize;
    }

    println!("{}", nmap[&min]);
}
