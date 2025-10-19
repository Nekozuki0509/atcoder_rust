use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
    }

    let mut fact = vec![1];
    for i in 1..=n {
        fact.push((fact[i - 1] * (i % MOD)) % MOD);
    }

    for i in 1..=n {
        let mut ans = 0;
        for m in 1..=(n + i - 1) / i {
            ans += ncr(n - (m - 1) * (i - 1), m, &fact);
            ans %= MOD;
        }

        println!("{}", ans);
    }
}

fn ncr(n: usize, r: usize, fact: &Vec<usize>) -> usize {
    division(fact[n], fact[r] * fact[n - r] % MOD)
}

fn division(a: usize, b: usize) -> usize {
    (a * modpow(b, MOD - 2, MOD)) % MOD
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
