#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

const MOD: usize = 998_244_353;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [[usize;6];n]
    }

    let mut an = 0usize;
    for i in a {
        let mut temp = 0usize;
        for j in a {
            temp += j;
            temp %= MOD;
        }

    }
}
