use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize;n],
        lr: [(isize, isize);q]
    }

    let mut b = vec![0];
    for i in a.iter().enumerate() {
        b.push(b[i.0] + i.1);
    }

    for i in lr {
        println!("{}", b[(i.1) as usize] - b[(i.0-1) as usize]);
    }
}
