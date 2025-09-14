use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize)
    }

    let mut an;
    if h == 1 || w == 1 {
        an = 1;
    } else {
        an = h * (w / 2);
        if w % 2 == 1 {
            an += h / 2 + h % 2;
        }
    }
    
    println!("{}", an);
}
