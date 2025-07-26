#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, l, r): (usize, usize, usize),
        p: [(usize, usize);n]
    }

    let mut an = 0usize;
    for (x, y) in p {
        if l >= x && r <= y {
            an += 1;
        }
    }

    println!("{}", an);
}
