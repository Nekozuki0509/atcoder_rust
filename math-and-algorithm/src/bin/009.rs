use nalgebra::coordinates::X;
#[allow(unused_imports)]
use proconio::{input, marker::{Bytes, Chars, Usize1}};

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize;n]
    }

    let mut v = vec![false; s+1];
    v[0] = true;

    for x in a {
        for i in (0..=s).rev() {
            if v[i] && x + i <= s {
                v[x+i] = true;
            }
        }
    }

    if v[s] {
        println!("Yes");
    } else {
        println!("No");
    }
}
