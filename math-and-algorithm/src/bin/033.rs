use libm::sqrt;
use proconio::input;

fn main() {
    input! {
        (a, b, c): ((isize, isize), (isize, isize), (isize, isize))
    }

    let (ba, bc) = ((a.0 - b.0, a.1 - b.1), (c.0 - b.0, c.1 - b.1));
    let (ca, cb) = ((a.0 - c.0, a.1 - c.1), (b.0 - c.0, b.1 - c.1));

    let an: f64 = 
    if ba.0 * bc.0 + ba.1 * bc.1 < 0 {
        sqrt((ba.0.pow(2) + ba.1.pow(2)) as f64)
    } else if ca.0 * cb.0 + ca.1 * cb.1 < 0 {
        sqrt((ca.0.pow(2) + ca.1.pow(2)) as f64)
    } else {
        (ba.0 * ca.1 - ba.1 * ca.0).abs() as f64 / sqrt((bc.0.pow(2) + bc.1.pow(2)) as f64)
    };
    
    println!("{}", an);
}
