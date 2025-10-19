use num_integer::sqrt;
use proconio::input;

fn main() {
    input! {
        (l, r): (usize, usize),
    }

    let mut v = vec![true; r - l + 1];
    if l == 1 {
        v[0] = false;
    }

    for i in 2..=sqrt(r) {
        for j in (((i * 2).max((l - 1) - (l - 1) % i + i))..=r).step_by(i) {
            v[j - l] = false;
        }
    }

    let mut ans = 0usize;
    for i in v {
        if i {
            ans += 1;
        }
    }

    println!("{}", ans);
}
