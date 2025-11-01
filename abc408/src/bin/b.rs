#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        a: usize,
        mut n: [usize;a]
    }

    n.sort();
    n.dedup();

    println!("{}", n.len());
    println!("{}", n.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "))
}
