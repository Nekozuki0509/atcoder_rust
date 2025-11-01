use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }

    s = s.iter().filter(|&c| !c.eq(&'a') && !c.eq(&'i') && !c.eq(&'u') && !c.eq(&'e') && !c.eq(&'o')).cloned().collect();
    

    let an: String = s.iter().collect();
    println!("{}", &an);
}
