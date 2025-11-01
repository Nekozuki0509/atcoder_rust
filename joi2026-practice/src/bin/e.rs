#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let mut an = 0usize;
    for i in s {
        match i {
            'j' => an += 2,
            'o' => an += 1,
            _ => an += 2
        }
    }

    println!("{}", an);
}
