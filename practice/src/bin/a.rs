use libs::common::gcd;
#[allow(unused_imports, non_snake_case)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        a: usize,
        (b, c): (usize, usize),
        s: String,
    }

    dbg!(a, b, gcd(a as u64, b as u64));

    println!("{} {}", a + b + c, s);
}
