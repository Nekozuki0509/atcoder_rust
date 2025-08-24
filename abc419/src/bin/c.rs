use std::collections::BTreeSet;

use libm::{ceil, round};
use num::abs;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize
    }

    let mut x = BTreeSet::new();
    let mut y = BTreeSet::new();
    for _ in 0..n {
        input! {r: isize, c: isize}
        x.insert(r);
        y.insert(c);
    }

    // let xm = x.iter().sum::<isize>() / x.len() as isize;
    // let ym = y.iter().sum::<isize>() / y.len() as isize;
    
    // dbg!(xm, ym);

    let mxd = ceil((x.last().unwrap()-x.first().unwrap()) as f64 / 2.);
    let myd = ceil((y.last().unwrap()-y.first().unwrap()) as f64 / 2.);
    // dbg!(x.last().unwrap(), x.first().unwrap());
    // dbg!(x.last().unwrap()-x.first().unwrap());
    // dbg!(mxd, myd);
    let ans1 = mxd.max(myd);
    
    // let mxd = md;
    // let myd = (abs(y.first().unwrap()-ym-1)).max(abs(y.last().unwrap()-ym-1));
    // // dbg!(mxd, myd);
    // let ans2 = mxd.max(myd);

    //dbg!(ans1, ans2);

    println!("{}", mxd.max(myd))
}
