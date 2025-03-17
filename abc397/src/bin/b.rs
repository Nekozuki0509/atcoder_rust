use proconio::{input, marker::Chars};

fn main() {
    // input! {
    //     mut s: Chars,
    // }

    // let mut an = 0;
    // let mut now = 1;
    
    // for c in s {
    //     if now == 1 && !c.eq(&'i') {
    //         an += 1;
    //         continue;
    //     } else if now == 1 {
    //         now = 2;
    //         continue;
    //     }

    //     if !c.eq(&'o') {
    //         an += 1;
    //     }
        
    //     now = 1;
    // }

    // if now == 2 {
    //     an += 1;
    // }

    input! {
        mut s: String,
    }

    s = s.replace("io", "");

    println!("{}", s.chars().count());
}
