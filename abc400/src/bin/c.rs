use std::collections::BTreeSet;

use num_integer::Roots;
#[allow(unused_imports)]
use proconio::{input, marker::{Bytes, Chars, Usize1}};

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
    }

    // let mut prime = vec![true;(n/2).sqrt()+1];

    // for i in 2..=(n/2).sqrt().sqrt() {
    //     if prime[i] {
    //         for j in (2*i..=(n/2).sqrt()).filter(|x| x % i == 0).into_iter() {
    //             prime[j] = false;
    //         }
    //     }
    // }

    // let mut primes = prime[2..].iter().enumerate().filter(|x| *x.1).map(|x| x.0 + 2).collect::<Vec<usize>>();
    // primes.push(1);
    //println!("{:?}", primes);
    //let mut set = BTreeSet::new();
    let mut an = 0;
    for i in 1..=(n/2).sqrt() {
        if i % 2 == 0 {
            continue;
        }
        
        //println!("i^2={}, log2={}", i.pow(2), ((n/i.pow(2)) as isize).ilog2());
        //println!("{}", n/i.pow(2));
        an += ((n/i.pow(2))).ilog2();
    }
    
    //println!("{:?}", set);
    //println!("{}", set.len());
    println!("{}", an);
}

