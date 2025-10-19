use num::{checked_pow, CheckedMul};
use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize)
    }

    if let Some(x) = lcm(a.max(b), a.min(b)) {
        println!("{}", x);
    } else {
        println!("Large");
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> Option<usize> {
    if let Some(x) = b.checked_mul(a / gcd(a, b)) {
        if x <= 1e18 as usize {
            Some(x)
        } else {
            None
        }
    } else {
        None
    }
}
