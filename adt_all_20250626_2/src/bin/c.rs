#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;2*n]
    }

    let mut an = 0usize;
    for i in 2..2*n {
        if a[i-2] == a[i] {
            an += 1;
        }
    }

    println!("{}", an);
}
