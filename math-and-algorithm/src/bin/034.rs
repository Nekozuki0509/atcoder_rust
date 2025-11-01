use libm::sqrt;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [(isize, isize);n]
    }

    let mut an = 2f64.powf(60.0);
    for i in a.iter().enumerate() {
        for j in a[i.0+1..].iter() {
            an = an.min(sqrt((((i.1).0 - j.0).pow(2) + ((i.1).1 - j.1).pow(2)) as f64));
        }
    }
    
    println!("{}", an);
}
