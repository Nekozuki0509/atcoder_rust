use proconio::input;

fn main() {
    input! {
        (a, b, c, d): (isize, isize, isize, isize)
    }
    
    println!("{}", ((a * c).max(a * d)).max((b * c).max(b * d)));
}
