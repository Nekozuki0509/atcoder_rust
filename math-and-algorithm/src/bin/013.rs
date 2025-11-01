use std::collections::BTreeSet;

use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    for i in 1..=n.nth_root(2) {
        if n % i == 0 {
            println!("{}", i);
            println!("{}", n / i);
        }
    }
}
