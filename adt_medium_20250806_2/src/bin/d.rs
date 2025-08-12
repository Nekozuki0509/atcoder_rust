use num::abs;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, l, r): (usize, isize, isize),
        a: [isize;n]
    }

    for i in a {
        if l <= i && i <= r {
            print!("{} ", i);
        } else if abs(i - l) < abs(i - r) {
            print!("{} ", l);
        } else {
            print!("{} ", r);
        }
    }
}
