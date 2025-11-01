use std::collections::BTreeMap;

use itertools::Itertools;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        s: [Chars;n]
    }

    let mut vec = vec![vec![];m];
    for (i, v) in s.iter().enumerate() {
        for (j, w) in v.iter().enumerate() {
            vec[j].push(*w);
        }
    }

    //dbg!(&vec);

    let mut map = BTreeMap::new();
    for i in vec {
        //dbg!(&i);
        let mut a = (vec![], vec![]);
        for (j, v) in i.iter().enumerate() {
            if v.eq(&'0') {
                a.0.push(j+1);
            } else {
                a.1.push(j+1);
            }
        }

        if a.0.len() == 0 || a.1.len() == 0 {
            for j in 1..=n {
                *map.entry(j).or_insert(0) += 1;
            }
        } else if a.0.len() > a.1.len() {
            for j in a.1 {
                *map.entry(j).or_insert(0) += 1;
            }
        } else {
            for j in a.0 {
                *map.entry(j).or_insert(0) += 1;
            }
        }

        //dbg!(&map);
    }

    let mut vec = vec![];

    let mut max = 0;
    for (i, v) in map {
        if max == v {
            vec.push(i);
        } else if max < v {
            vec = vec![i];
            max = v;
        }
    }

    println!("{}", vec.iter().map(|x| x.to_string()).join(" "))
}
