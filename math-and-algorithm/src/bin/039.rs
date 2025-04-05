use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        lrx: [(isize, usize, isize);q]
    }

    let mut b: Vec<isize> = vec![0];
    for _ in 0..n {
        b.push(0);
    }

    for i in lrx {
        b[(i.0-1) as usize] += i.2;
        b[i.1] -= i.2;
    }

    for i in b[1..n].iter() {
        let an = 
        if i > &0 {
            "<"
        } else if i == &0 {
            "="
        } else {
            ">"
        };

        print!("{}", an);
    }
}
