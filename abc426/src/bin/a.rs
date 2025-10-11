#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (x, y): (String, String)
    }

    if y.eq(&"Ocelot") {
        println!("Yes");
    } else if y.eq(&"Serval") {
        if !x.eq(&"Ocelot") {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        if x.eq(&"Lynx") {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
