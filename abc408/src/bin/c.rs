#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
    }

    let mut b = vec![0isize;n+2];
    for _ in 0..m {
        input! {(l, r): (usize, usize)}
        b[l] += 1;
        b[r+1] -=1;
    }

    let mut an = 2isize.pow(60);
    let mut all = 0isize;
    //dbg!(&b);
    for i in b[1..=n].iter() {
        all += i;
        an = an.min(all);
        //dbg!(i, all);
    }

    println!("{}", an);
}
