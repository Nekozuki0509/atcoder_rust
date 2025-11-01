use proconio::input;

fn main() {
    input! {
        (a, b, c): (isize, isize, isize),
    }

    let ans = 
        if c - a - b < 0 {"No"}
        else if 4 * a * b < (c - a - b).pow(2) {"Yes"}
        else {"No"};
    
    println!("{}", ans);
}
