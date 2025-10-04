use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut now = 0;
    for i in s {
        if i.eq(&'(') {
            now += 1;
        } else if now == 0 {
            println!("No");
            return;
        } else {
            now -= 1;
        }
    }
    
    println!("Yes");
}
