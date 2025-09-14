#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {mut a: usize, b: usize, mut c: usize}

        if b >= a.min(c) {
            println!("{}", a.min(c));
            continue;
        }

        a -= b;
        c -= b;

        println!("{}", b + ((a+c)/3).min(a.min(c)));
    }
}
