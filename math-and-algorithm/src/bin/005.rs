use std::usize;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n],
    }
    
    println!("{}", a.iter().sum::<usize>() % 100);
}
