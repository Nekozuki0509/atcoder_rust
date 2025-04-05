use libm::sqrt;
use proconio::input;

fn main() {
    input! {
        o1: ((isize, isize), f64),
        o2: ((isize, isize), f64)
    }

    let l = sqrt((((o1.0).0 - (o2.0).0).pow(2) + ((o1.0).1 - (o2.0).1).pow(2)) as f64);
    
    let an =
    if l == (o2.1 - o1.1).abs() {
        "2"
    } else if l < (o2.1 - o1.1).abs() {
        "1"
    } else if l > o1.1 + o2.1 {
        "5"
    } else if l == o1.1 + o2.1 {
        "4"
    } else {
        "3"
    };
    
    println!("{}", an);
}
