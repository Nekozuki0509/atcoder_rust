use proconio::input;

fn main() {
    input! {
        a: [usize;3],
    }
    
    println!("{}", a.iter().fold(1, |acc, &x| acc * x));
}
