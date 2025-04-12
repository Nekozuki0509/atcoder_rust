#[allow(unused_imports)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[allow(non_snake_case)]
//#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n]
    }

    let mut login = false;
    let mut an = 0;
    for i in s {
        match &i[..] {
            "login" => login = true,
            "logout" => login = false,
            "public" => {},
            "private" => {
                if !login {
                    an += 1;
                }
            },
            &_ => {}
        }
    }

    
    
    println!("{}", an);
}
