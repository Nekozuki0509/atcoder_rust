#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (t, m): (usize, usize)
    }

    let mut vec = vec![1];
    for i in 1..=5000 {
        vec.push((vec[i-1] * (i % m))%m);
    }

    for _ in 0..t {
        input! {n: usize, c: [usize;n]}
        let mut mul = 1;
        let mut sum = 0;
        for i in c {
            mul *= vec[i];
            mul %= m;
            sum += i;
        }
        dbg!(sum, division(sum, mul, m));

        println!("{}", division(vec[sum], mul, m));
    }
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
