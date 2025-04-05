use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [f64;n],
        b: [f64;n]
    }

    let mut an = 0.0;
    for i in 0..n {
        an += a[i] / 3f64 + b[i] / 3f64 * 2f64;
    }
    
    println!("{}", an);
}
