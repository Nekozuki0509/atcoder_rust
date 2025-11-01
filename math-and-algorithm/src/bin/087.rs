use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
    }
    
    println!("{}", ((((n%MOD) * ((n%MOD)+1)) / 2)%MOD).pow(2)%MOD);
}
