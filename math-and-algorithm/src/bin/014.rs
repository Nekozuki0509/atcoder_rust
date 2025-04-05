use num_integer::Roots;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        mut n: usize,
    }

    for i in 2..=n.sqrt() {
        while n % i == 0 {
            print!("{} ", i);
            n /= i;
        }

        if n == 1 {
            return;
        }
    }

    if n > 1 {
        println!("{}", n);
    }
}