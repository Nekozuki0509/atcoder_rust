use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: u32,
        s: Chars,
    }

    for i in 1..=n-1 {
        for j in 1..=n-i {
            if i + j == n {
                println!("a{}", n - 1);
                break;
            }
            if s[j as usize].eq(&s[(j + i) as usize]) {
                println!("b{}", j as i32 - 1);
                break;
            }
        }
    }
}
