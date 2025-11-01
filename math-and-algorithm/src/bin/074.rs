use proconio::input;

fn main() {
    input! {
        n: isize,
        a: [isize;n]
    }
    
    let mut an = 0;
    for (i, &v) in a.iter().enumerate() {
        an += v * (2*(i as isize)+1-n);
    }

    println!("{}", an);
}
