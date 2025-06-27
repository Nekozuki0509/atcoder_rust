use num::Integer;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (a, b): (usize, usize)
    }

    println!("{}", (a).div_ceil(&b));
}
