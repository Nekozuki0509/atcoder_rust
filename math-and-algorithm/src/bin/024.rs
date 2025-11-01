use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [(f64, f64);n],
    }

    let mut an = 0.0;
    for i in a {
        an += i.1 / i.0;
    }
    
    println!("{}", an);
}
