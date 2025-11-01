use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    if c == 1 {
        println!("No");
        return;
    }

    let mut v = 1;
    for _ in 1..=b {
        if a / c < v {
            println!("Yes");
            return;
        }

        v *= c;
    }
    
    println!("No");
}
