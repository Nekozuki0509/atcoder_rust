use num_integer::sqrt;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut m = 1usize << 60;
    for i in 1..=sqrt(n) {
        if n % i == 0 && i + n / i < m {
            m = i + n / i;
        }
    }

    println!("{}", m * 2);
}
