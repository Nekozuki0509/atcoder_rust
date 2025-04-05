use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    
    println!("{}", (1..=n).into_iter().fold(0.0, |acc, x| acc + n as f64 / x as f64));
}
