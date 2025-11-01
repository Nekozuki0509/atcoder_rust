use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (x, y): (usize, usize),
    }

    let (mut bunshi, mut bunbo, modv) = (1, 1, 1000000007);
    let mut an = 0;
    for i in 0..=x/2 {
        if y == i + 2 * (x-i*2) {
            for i in 1..=x-i {
                bunshi *= i;
                bunshi %= modv;
            }
            for i in 1..=x-i*2 {
                bunbo *= i;
                bunbo %= modv;
            }
            for i in 1..=i {
                bunbo *= i;
                bunbo %= modv;
            }

            an += division(bunshi, bunbo, modv);
            an %= 1_000_000_007;
        }
    }

    println!("{}", an);
}

fn modpow(mut a: usize, b: usize, m: usize) -> usize {
    let mut an = 1usize;
    for i in 0..30 {
        if (b & (1 << i)) != 0 {
            an *= a;
            an %= m;
        }

        a *= a;
        a %= m;
    }

    an
}

fn division(a: usize, b: usize, m: usize) -> usize {
    (a * modpow(b, m - 2, m)) % m
}
