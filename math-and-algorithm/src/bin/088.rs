use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        (a, b, c): (usize, usize, usize)
    }
    
    println!("{}", (((((((a%MOD) * ((a%MOD)+1)) / 2)%MOD) * ((((b%MOD) * ((b%MOD)+1)) / 2)%MOD))%MOD)*((((c%MOD) * ((c%MOD)+1)) / 2)%MOD))%MOD);
}
