use num_integer::Roots;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for i in 0..t {
        input! {ts: (isize, isize), tg: (isize, isize), os: (isize, isize), og: (isize, isize)}
        if intersect(ts, tg, os, og) {
            println!("{}", 0.);
        } else {
            let s = ((ts.0 - os.0).pow(2) + (ts.1 - os.1).pow(2)).sqrt();
            let g = ((tg.0 - og.0).pow(2) + (tg.1 - og.1).pow(2)).sqrt();
            println!("{}", s.min(g));
        }
    }
}

// 線分p1p2とp3p4が交わるかどうかを判定する
fn intersect(p1: (isize, isize), p2: (isize, isize), p3: (isize, isize), p4: (isize, isize)) -> bool {
    outer_product(p1, p2, p3) * outer_product(p1, p2, p4) < 0
        && outer_product(p3, p4, p1) * outer_product(p3, p4, p2) < 0
}

// p1p2とp1p3の外積
fn outer_product(p1: (isize, isize), p2: (isize, isize), p3: (isize, isize)) -> isize {
    let a = p2.0 - p1.0;
    let b = p2.1 - p1.1;
    let c = p3.0 - p1.0;
    let d = p3.1 - p1.1;
    a * d - b * c
}