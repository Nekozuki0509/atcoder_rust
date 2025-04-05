use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize);n]
    }

    let mut d = vec![0isize;t+1];

    for i in lr {
        d[i.0] += 1;
        d[i.1] -= 1;
    }

    let mut temp = 0;
    for i in d[..t].iter() {
        temp += i;
        println!("{}", temp);
    }
}
