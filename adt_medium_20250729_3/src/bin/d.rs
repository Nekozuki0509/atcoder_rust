#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

const MOD: usize = 998244353;

#[fastout]
fn main() {
    input! {
        n: [usize;6]
    }

    println!("{}", (((((((n[0] % MOD) * (n[1] % MOD)) % MOD) * (n[2] % MOD)) % MOD) + MOD) - (((((n[3] % MOD) * (n[4] % MOD)) % MOD) * (n[5] % MOD)) % MOD) % MOD) % MOD)
}
