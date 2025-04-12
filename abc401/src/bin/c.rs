#[allow(unused_imports)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[allow(non_snake_case)]
//#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize)
    }

    if n < k {
        println!("1");
        return;
    }

    let mut a = vec![1; k];
    a.push(k);
    for i in k+1..=n {
        a.push((a[i-1] * 2 + 1_000_000_000 - a[i-k-1]) % 1_000_000_000);
    }
    
    println!("{}", a[n]);
}
