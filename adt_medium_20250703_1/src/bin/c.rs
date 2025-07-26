
use num::abs;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (x, y): (isize, isize)
    }

    let mut an = 0.;
    for i in 1..=6isize {
        for j in 1..=6 {
            if i + j >= x || abs(i - j) >= y {
                an += 1.;
            }
        }
    }

    println!("{}", an / 36.);
}
