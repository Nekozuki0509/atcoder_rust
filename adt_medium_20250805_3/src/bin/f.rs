#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        a: [[char;9];9],
    }

    let mut column = vec![vec![];9];
    let mut row = vec![vec![];9];
    let mut nine = vec![vec![vec![];3];3];
    for (i, v) in a.iter().enumerate() {
        for (j, w) in v.iter().enumerate() {
            //dbg!(w);
            if row[i].contains(w) || column[j].contains(w) || nine[i/3][j/3].contains(w) {
                println!("No");
                return;
            } else {
                row[i].push(*w);
                column[j].push(*w);
                nine[i/3][j/3].push(*w);
            }
        }
    }

    println!("Yes");
    //dbg!(nine);
}
