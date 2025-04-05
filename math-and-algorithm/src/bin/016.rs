use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }

    let mut r = 0;
    for mut i in a {
        while i != 0 && r != 0 {
            if i >= r {
                i %= r;
            } else {
                r %= i;
            }
        }

        r = r.max(i);
    }
    
    println!("{}", r);
}
