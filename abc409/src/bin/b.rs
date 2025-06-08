#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }

    let mut vec = vec![0isize;*a.iter().max().unwrap()+2];
    for i in a {
        vec[0] += 1;
        vec[i+1] -= 1;
    }

    //dbg!(&vec);

    let mut nvec = vec![];
    for (i, &v) in vec.iter().enumerate() {
        if i == 0 {
            nvec.push(v);
            continue;
        }

        nvec.push(nvec[i-1] + v);
    }

    for (i, &v) in nvec.iter().enumerate().rev() {
        if v >= i as isize {
            println!("{}", i);
            return;
        }
    }
}
