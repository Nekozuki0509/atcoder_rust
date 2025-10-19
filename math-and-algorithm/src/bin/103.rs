use libm::sqrt;
use proconio::input;

fn main() {
    let t = 1. / sqrt(2.);
    let r = t / 10.;
    for i in (1..=20).step_by(2) {
        for j in (1..=20).step_by(2) {
            println!("{} {}", r * j as f64 - t, r * i as f64 - t);
        }
    }
}
