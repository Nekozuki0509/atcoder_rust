#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize;n]
    }
    
    a.sort();
    a.dedup();

    let mut v = vec![false;200_100];
    for &i in &a {
        v[i] = true;
    }

    for i in a {
        if v[i + 3] && v[i + 6] {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
