use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut an = vec!["-";n];
    if n % 2 == 0 {
        an[n/2-1] = "=";
        an[n/2] = "=";
    } else {
        an[(n+1)/2-1] = "=";
    }
    
    println!("{}", an.concat());
}
