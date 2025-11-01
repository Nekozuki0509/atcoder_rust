#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars
    }

    //let nt = t.iter().map(|&x| x.to_lowercase().to_string()).collect::<Vec<String>>();
    for (i, v) in s.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if v.is_uppercase() {
            if !t.contains(&s[i-1]) {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
