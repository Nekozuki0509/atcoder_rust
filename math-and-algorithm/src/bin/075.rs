use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }

    let mut fact = vec![1usize];
    for i in 1usize..=200000 {
        fact.push(fact[i-1] * i % MOD);
    }

    let mut an = 0;
    for (i, &v) in a.iter().enumerate() {
        an += v * ncr(n-1, i, &fact);
        an %= MOD;
    }
    
    println!("{}", an);
}

fn ncr(n: usize, r: usize, fact: &Vec<usize>) -> usize {
    division(fact[n], fact[r] * fact[n-r] % MOD)
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

