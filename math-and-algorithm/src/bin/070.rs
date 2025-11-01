use proconio::input;

fn main() {
    input! {
        (n, t): (usize, usize),
        p: [(isize, isize);n]
    }

    let mut an = (1isize << 62);
    for i in p.iter().map(|&(x, _)| x) {
        for j in p.iter().map(|&(x, _)| x) {
            for k in p.iter().map(|&(_, y)| y) {
                for l in p.iter().map(|&(_, y)| y) {
                    if check_numpoints(n, &p, i, j, k, l) >= t {
                        an = an.min((j - i) * (l - k));
                    }
                }
            }
        }
    }
    
    println!("{}", an);
}

fn check_numpoints(n: usize, p: &Vec<(isize, isize)>, lx: isize, rx: isize, ly: isize, ry: isize) -> usize {
    let mut cnt = 0usize;
    for i in 0..n {
        if lx <= p[i].0 && p[i].0 <= rx && ly <= p[i].1 && p[i].1 <= ry {
            cnt += 1;
        }
    }

    cnt
}
