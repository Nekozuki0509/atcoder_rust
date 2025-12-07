use ac_library::ModInt;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (t, m): (usize, u32)
    }

    ModInt::set_modulus(m);
    let mut vec = vec![ModInt::new(1)];
    for i in 1..=5000 {
        vec.push(vec[i-1] * ModInt::new(i));
    }

    for _ in 0..t {
        input! {n: usize, c: [usize;n]}
        let mut mint = ModInt::new(1usize);
        let mut sum = 0;
        for i in c {
            mint *= vec[i];
            sum += i;
        }
        println!("{}", vec[sum] / mint);
    }
}
