use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize
    }

    while a != 0 && b != 0 {
        if a >= b {
            a %= b;
        } else {
            b %= a;
        }
    }
    
    println!("{}", a.max(b));
}
