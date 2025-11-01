use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }
    
    let mut an = 0;
    let mut mul = 1usize;
    for v in a {
        an += ((v % MOD) * mul) % MOD;
        an %= MOD;
        mul *= 2;
        mul %= MOD;
    }

    println!("{}", an);
}
