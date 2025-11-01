#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

struct Query {
    t: u8,
    p: usize,
    s: Option<String>
}

#[fastout]
fn main() {
    input! {
        (_n, q): (usize, usize)
    }

    let mut vec = vec![];

    for _ in 0..q {
        input! {t: u8, p: usize}
        match t {
            1 => {
                vec.push(Query { t: 1, p, s: None });
            },
            2 => {
                input! {s: String}
                vec.push(Query { t: 2, p, s: Some(s) });
            },
            3 => {
                vec.push(Query { t: 3, p, s: None });
            },
            _ => unreachable!()
        }
    }

    vec.reverse();
    let mut is = vec![];

    let mut flag = true;
    let mut search = 0usize;
    for i in vec.into_iter() {
        if flag && i.t == 3 {
            search = i.p;
            is.push(i);
            flag = false;
            continue;
        }

        if i.p == search {
            is.push(i);
        }
    }

    let mut ps = String::new();
    let mut ss = String::new();
    for i in is.into_iter().rev() {
        match i.t {
            1 => {
                ps = ss.clone();
            },
            2 => {
                ps.push_str(&(i.s.unwrap()));
            },
            3 => {
                ss = ps.clone();
            },
            _ => unreachable!()
        }
    }

    println!("{}", ss);
}
