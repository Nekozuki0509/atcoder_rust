use proconio::input;

fn main() {
    input! {
        n: isize,
        mut a: [isize;n]
    }

    a.sort();

    let mut an = 0;
    for (i, &v) in a.iter().enumerate() {
        an += v * (2*(i as isize)+1-n);
    }
    
    println!("{}", an);
}
