#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut l = vec![0];

    for (i, v) in a.iter().enumerate() {
        l.push(l[i] + v);
    }

    let mut an = 0;
    for (i, v) in a.iter().enumerate() {
        an += v * (l[a.len()] - l[i+1]);
    }
    
    println!("{}", an);
}
