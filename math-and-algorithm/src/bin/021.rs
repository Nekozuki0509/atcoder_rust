use proconio::input;

fn main() {
    input! {
        n: usize,
        r: usize
    }
    
    println!("{}", (n-r+1..=n).fold(1, |acc, x| acc * x) / (2..=r).fold(1, |acc, x| acc * x));
}
