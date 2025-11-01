use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize
    }

    let (mut bunshi, mut bunbo, modv) = (1, 1, 1000000007);
    for i in 1..=x + y {
        bunshi *= i;
        bunshi %= modv;
    }
    for i in 1..=x {
        bunbo *= i;
        bunbo %= modv;
    }
    for i in 1..=y {
        bunbo *= i;
        bunbo %= modv;
    }
    
    println!("{}", division(bunshi, bunbo, modv));
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