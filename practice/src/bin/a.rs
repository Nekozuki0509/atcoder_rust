use libs::common::{gcd, lcm};
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        a: usize,
        (b, c): (usize, usize),
        s: String
    }

    dbg!(gcd(57, 3));
    dbg!(lcm(3, 19));

    println!("{} {}", a + b + c, s);
}
