use proconio::input;

const MOD: usize = 1_000_000_007;

#[derive(Debug, Clone)]
struct Matrix {
    p: Vec<Vec<usize>>
}

impl Matrix {
    fn new(n: usize) -> Self {
        Self { p: vec![vec![0;n];n] }
    }
}

fn mul(a: &Matrix, b: &Matrix) -> Matrix {
    let n = a.p.len();
    let mut c = Matrix::new(n);
    for i in 0..n {
        for k in 0..n {
            for j in 0..n {
                c.p[i][j] += a.p[i][k] * b.p[k][j];
                c.p[i][j] %= MOD;
            }
        }
    }
    
    c
}

fn pow(a: &mut Matrix, n: usize) -> Matrix {
    let mut q = Matrix::new(a.p.len());
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

fn build_transition(h: usize) -> Matrix {
    let dim = 1usize << h;
    let mut mat = Matrix::new(dim);

    fn dfs(h: usize, row: usize, cur_mask: usize, next_mask: usize, used: usize, mat: &mut Matrix) {
        if row == h {
            mat.p[cur_mask][next_mask] += 1;
            return;
        }

        if (used >> row) & 1 == 1 {
            dfs(h, row + 1, cur_mask, next_mask, used, mat);
            return;
        }

        if row + 1 < h && (used >> (row + 1)) & 1 == 0 {
            dfs(h, row + 1, cur_mask, next_mask, used | (1 << row) | (1 << (row + 1)), mat);
        }

        dfs(h, row + 1, cur_mask, next_mask | (1 << row), used | (1 << row), mat);
    }

    for cur in 0..dim {
        dfs(h, 0, cur, 0, cur, &mut mat);
    }

    mat
}

fn main() {
    input! {
        (k, n): (usize, usize)
    }
    
    println!("{}", pow(&mut build_transition(k), n).p[0][0]);
}
