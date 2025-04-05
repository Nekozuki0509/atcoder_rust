use std::collections::BTreeSet;

use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut v = vec![0;n+1];
    for i in 1..=n {
        let mut j = i;
        while j <= n {
            v[j] += 1;
            j += i;            
        }
    }
    
    println!("{}", v.iter().enumerate().map(|x| x.0 * x.1).sum::<usize>());
}