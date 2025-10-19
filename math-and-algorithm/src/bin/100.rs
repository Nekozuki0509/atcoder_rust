use proconio::{fastout, input};

#[derive(Debug, Clone)]
struct Matrix {
    p: Vec<Vec<f64>>,
}

impl Matrix {
    fn new() -> Self {
        Self {
            p: vec![vec![0.; 3]; 3],
        }
    }

    fn mul(&self, other: &Self) -> Self {
        let mut tmp = Matrix::new();
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    tmp.p[i][j] += self.p[i][k] * other.p[k][j];
                }
            }
        }

        tmp
    }

    fn pow(&self, n: usize) -> Self {
        let mut a = self.clone();
        let mut q = Matrix::new();
        let mut flag = false;

        for i in 0..25usize {
            if (n & (1 << i)) != 0 {
                if flag {
                    q = q.mul(&a);
                    //dbg!(&q);
                } else {
                    q = a.clone();
                    flag = true;
                }
            }

            a = a.mul(&a);
        }

        q
    }
}

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    for _ in 0..q {
        input! {x: f64, y: f64, z: f64, t: usize}

        let mut m = Matrix::new();
        m.p[0][0] = 1. - x;
        m.p[0][1] = y;
        m.p[1][1] = 1. - y;
        m.p[1][2] = z;
        m.p[2][0] = x;
        m.p[2][2] = 1. - z;

        let m = m.pow(t);
        println!(
            "{} {} {}",
            m.p[0].iter().sum::<f64>(),
            m.p[1].iter().sum::<f64>(),
            m.p[2].iter().sum::<f64>()
        );
    }
}
