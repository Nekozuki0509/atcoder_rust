use num_integer::lcm;
use proconio::input;

fn main() {
    input! {
        (n, k): (isize, usize),
        mut v: [isize;k]
    }

    let mut an = 0;
    for i in 1usize..(1 << k) {
        let mut m = 1;
        let mut p = -1;
        for j in 0..k {
            if (1 << j) & i != 0 {
                m = lcm(m, v[j]);
                p *= -1;
            }
        }

        an += n / m * p;
    }
    
    println!("{}", an);
}
