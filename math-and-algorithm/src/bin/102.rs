use proconio::{
    input,
    marker::{Chars, Isize1},
};

fn main() {
    input! {
        n: Isize1,
        c: Chars
    }

    let mut sum = 0;
    for (i, &v) in c.iter().enumerate() {
        let t = match v {
            'B' => 0,
            'W' => 1,
            _ => 2,
        };

        sum += ncr(n, i as isize) * t;
        sum %= 3;
    }

    if n % 2 == 1 {
        sum = (3 - sum) % 3;
    }

    let ans = match sum {
        0 => "B",
        1 => "W",
        _ => "R",
    };

    println!("{}", ans);
}

fn ncr(n: isize, r: isize) -> isize {
    if n < 3 && r < 3 {
        if n < r {
            0
        } else if n == 2 && r == 1 {
            2
        } else {
            1
        }
    } else {
        ncr(n / 3, r / 3) * ncr(n % 3, r % 3) % 3
    }
}
