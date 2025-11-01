use libm::sqrt;
use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64
    }

    let hr = (h * 30. + m / 2.).to_radians();
    let mr = (m * 6.).to_radians();

    let hv = (a * f64::cos(hr), a * f64::sin(hr));
    let mv = (b * f64::cos(mr), b * f64::sin(mr));

    let sv = (hv.0 - mv.0, hv.1 - mv.1);

    println!("{}", sqrt(sv.0.powf(2.) + sv.1.powf(2.)));
}
