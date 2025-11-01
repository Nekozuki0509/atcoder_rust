#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (x, y): (usize, usize)
    }

    let mut vec = vec![x, y];
    for i in 2..10 {
        vec.push((vec[i-1]+vec[i-2]).to_string().chars().into_iter().rev().collect::<String>().parse().unwrap());
    }

    println!("{}", vec[9])
}
