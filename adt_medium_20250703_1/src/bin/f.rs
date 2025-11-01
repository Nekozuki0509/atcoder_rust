use std::collections::BTreeSet;

#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (h, w): (usize, usize),
        s: [Chars;h]
    }

    let mut set = BTreeSet::new();
    let mut xmin = h;
    let mut xmax = 0usize;
    let mut ymin = w;
    let mut ymax = 0usize;

    for (i, s) in s.iter().enumerate() {
        for (j, c) in s.iter().enumerate() {
            if c.eq(&'#') {
                set.insert((i + 1, j + 1));
                xmin = xmin.min(i+1);
                xmax = xmax.max(i+1);
                ymin = ymin.min(j+1);
                ymax = ymax.max(j+1);
            }
        }
    }

    for x in xmin..=xmax {
        for y in ymin..=ymax {
            if !set.contains(&(x, y)) {
                println!("{} {}", x, y);
                return;
            }
        }
    }
}
