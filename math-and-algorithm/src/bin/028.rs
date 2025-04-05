use nalgebra::min;
use num::abs;
use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [isize;n]
    }

    let mut v = vec![];
    v.push(0);
    v.push(abs(h[0] - h[1]));
    for i in 2..n {
        v.push(min(v[i-1] + abs(h[i-1] - h[i]), v[i-2] + abs(h[i-2] - h[i])));
    }
    
    println!("{}", v[n-1]);
}
