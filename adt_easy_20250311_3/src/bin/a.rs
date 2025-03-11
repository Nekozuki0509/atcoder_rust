use proconio::input;

fn main() {
    input! {
        n: u32,
        b: [u32; n],
    }

    let mut i = 1;
    let mut f = 0;
    for h in b {
        if i == 1 {
            i += 1;
            f = h;
            continue;
        }

        if h > f {
            println!("{}", i);
            std::process::exit(0);
        }

        i += 1;
    }

    println!("-1");
}
