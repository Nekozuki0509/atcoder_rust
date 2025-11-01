use proconio::input;

fn main() {
    input! {
        n: usize,
        mut t: [(usize, usize);n]
    }

    t.sort_by_key(|&(l, r)| r);

    let mut now = 0;
    let mut ans = 0;
    for (l, r) in t {
        if l >= now {
            now = r;
            ans += 1;
        }
    }
    
    println!("{}", ans);
}
