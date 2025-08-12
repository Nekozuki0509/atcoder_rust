use itertools::Itertools;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        mut a: [usize;n],
        b: [usize;m]
    }

    for i in b {
        if a.contains(&i) {
            a.remove(a.iter().position(|&x| x == i).unwrap());
        }
    }

    println!("{}", a.iter().map(|x| x.to_string()).join(" "));
}
