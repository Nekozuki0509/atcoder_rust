use itertools::Itertools;
use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut prime = vec![true;n+1];

    for i in 2..=n.sqrt() {
        if prime[i] {
            for j in (2*i..=n).filter(|x| x % i == 0).into_iter() {
                prime[j] = false;
            }
        }
    }

    println!("{}", prime[2..].iter().enumerate().filter(|x| *x.1).map(|x| x.0 + 2).join(" "));
}
