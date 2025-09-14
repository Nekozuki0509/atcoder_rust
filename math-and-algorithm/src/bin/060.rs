use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    
    println!("{}", if n % 4 == 0 {"Second"} else {"First"});
}
