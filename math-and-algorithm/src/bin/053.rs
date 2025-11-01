use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let m = 10usize.pow(9) + 7;

    let a = modpow(4, n + 1, m) - 1;
    
    println!("{}", division(a, 3, m));
}

fn modpow(mut a: usize, b: usize, m: usize) -> usize {
    let mut an = 1usize;
    for i in 0..60 {
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