use num::abs;
use proconio::input;

fn main() {
    input! {
        (n, x, y): (isize, isize, isize),
    }
    
    println!("{}", if n >= abs(x) + abs(y) && n % 2 == (abs(x) + abs(y)) % 2 {"Yes"} else {"No"});
}
