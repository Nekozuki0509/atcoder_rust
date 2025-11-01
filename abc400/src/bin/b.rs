#[allow(unused_imports)]
use proconio::{input, marker::{Bytes, Chars, Usize1}};

#[proconio::fastout]
fn main() {
    input! {
        n: u128,
        m: usize
    }

    let mut an = 0;
    for i in 0..=m {
        if let Some(x) = n.checked_pow(i as u32) {
            an += x;
        } else {
            println!("inf");
            return;
        }
    }
    
    println!("{}", if an <= 10u128.pow(9) {an.to_string()} else {"inf".to_string()});
}
