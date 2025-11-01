#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        s: [usize;8],
    }

    let mut last = 0usize;
    for i in s {
        if i < last || i < 100 || i > 675 || i % 25 != 0 {
            println!("No");
            return;
        } else {
            last = i;
        }
    }

    println!("Yes");
}
