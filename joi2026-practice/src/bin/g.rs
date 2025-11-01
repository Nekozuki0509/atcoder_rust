#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        a: [usize;n],
        b: [usize;m]
    }

    let mut an = 0usize;
    for (i, a) in a.iter().enumerate() {
        for (j, b) in b.iter().enumerate() {
            if a <= b {
                an += 1;
            }
        }
    }

    println!("{}", an);
}
