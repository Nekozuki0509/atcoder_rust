use ascii::ToAsciiChar;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {n: usize, s: String}
        let mut vec = s.as_bytes();

        if vec.len() < 2 {
            println!("{}", s);
            continue;
        }
        let max = vec[..vec.len()-2].iter().max().unwrap();

        let mut flag = false;
        let mut an = String::new();
        for i in vec {
            if i == max && !flag {
                flag = true;
                continue;
            }

            an.push(*i as char);
        }

        an.push(*max as char);

        println!("{}", an);
    }
}
