use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [(isize, isize, isize);n]
    }

    let mut an = 0.;
    for (i, &(x1, y1, z1)) in p.iter().enumerate() {
        for &(x2, y2, z2) in p[i+1..].iter() {
            let x_gcd = gcd(x1, x2);
            let x_s_mul = x2 / x_gcd;
            let x_n_mul = x1 / x_gcd;
            let y = (z1 * x_s_mul - z2 * x_n_mul) as f64 / (y1 * x_s_mul - y2 * x_n_mul) as f64;
            
            let y_gcd = gcd(y1, y2);
            let y_s_mul = y2 / y_gcd;
            let y_n_mul = y1 / y_gcd;
            let x = (z1 * y_s_mul - z2 * y_n_mul) as f64 / (x1 * y_s_mul - x2 * y_n_mul) as f64;

            let mut flag = true;
            for &(a, b, c) in &p {
                if a as f64 * x + b as f64 * y > c as f64 {
                    flag = false;
                }
            }

            if flag {
                if an < x + y {
                    an = x + y;
                }
            }
        }
    }

    println!("{}", an);
}

fn gcd(a: isize, y2: isize) -> isize {
    if y2 == 0 {
        a
    } else {
        gcd(y2, a % y2)
    }
}