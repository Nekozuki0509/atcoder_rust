#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize
    }

    let mut c = vec![vec![];n];
    for i in 0..n {
        for _ in 0..n {
            input! {s: char}
            c[i].push(s);
        }
    }

    let mut vec = vec![vec![];n];
    for i in c {
        let x = i[0];
        let mut flag = true;
        //dbg!(&i);
        for (j, &v) in i.iter().enumerate() {
            if flag && !x.eq(&v) {
                flag = false;
            }
            vec[j].push(v);
        }
        //dbg!(flag);
        if flag {
            println!("Yes");
            return;
        }
    }

    for i in vec {
        let x = i[0];
        let mut flag = true;
        //dbg!(&i);
        for (j, &v) in i.iter().enumerate() {
            if flag && !x.eq(&v) {
                flag = false;
                break;
            }
        }
        //dbg!(flag);
        if flag {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
