use proconio::input;

const MOD: isize = 1_000_000_007;

#[derive(Debug, Clone)]
struct Matrix {
    p: Vec<Vec<isize>>
}

impl Matrix {
    fn new() -> Self {
        Self { p: vec![vec![0;3];3] }
    }
}

fn mul(a: &Matrix, b: &Matrix) -> Matrix {
    let mut c = Matrix::new();
    for i in 0..3 {
        for k in 0..3 {
            for j in 0..3 {
                c.p[i][j] += a.p[i][k] * b.p[k][j];
                c.p[i][j] %= MOD;
            }
        }
    }
    
    c
}

fn pow(a: &mut Matrix, n: usize) -> Matrix {
    let mut q = Matrix::new();
    let mut flag = false;

    for i in 0..60usize {
        if (n & (1 << i)) != 0 {
            if !flag {
                q = a.clone();
                flag = true;
            } else {
                q = mul(&q, &a);
            }
        }

        *a = mul(&a, &a);
    }

    q
}

fn main() {
    input! {
        n: usize,
    }

    let mut a = Matrix::new();
    a.p[0][1] = 1;
    a.p[1][2] = 1;
    a.p[2][0] = 1;
    a.p[2][1] = 1;
    a.p[2][2] = 1;

    let b = pow(&mut a, n - 1);
    
    println!("{}", (b.p[0][0] + b.p[0][1] + b.p[0][2] * 2) % MOD);

    //dbg!(b);
}
