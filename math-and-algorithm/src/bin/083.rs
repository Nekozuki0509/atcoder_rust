use num::abs;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [isize;n],
        mut b: [isize;n],
    }

    a.sort();
    b.sort();

    let mut ans = 0;
    for (&a, b) in a.iter().zip(b) {
        ans += abs(a - b)
    }
    
    println!("{}", ans);
}
