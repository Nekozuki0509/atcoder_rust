use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    
    println!("{}", if n + 1 == 2usize.pow(format!("{:b}", n).len() as u32) {"Second"} else {"First"});
}
