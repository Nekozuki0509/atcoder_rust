use itertools::Itertools;
use proconio::input;
use lazy_static::lazy_static;
use std::sync::Mutex;


lazy_static! {
    static ref A: Mutex<Vec<usize>> = Mutex::new(vec![]);

    static ref C: Mutex<Vec<usize>> = Mutex::new(vec![]);
}

fn merge_sort(l: usize, r: usize) {

    if r - l == 1 {
        return;
    }

    let m = (l + r) / 2;
    merge_sort(l, m);
    merge_sort(m, r);

    let (mut c1, mut c2, mut cnt) = (l, m, 0);
    let mut a = A.lock().unwrap();
    let mut c = C.lock().unwrap();
    while c1 != m || c2 != r {
        if c1 == m {
            c[cnt] = a[c2];
            c2 += 1;
        } else if c2 == r {
            c[cnt] = a[c1];
            c1 += 1;
        } else {
            if a[c1] < a[c2] {
                c[cnt] = a[c1];
                c1 += 1;
            } else {
                c[cnt] = a[c2];
                c2 += 1;
            }
        }

        cnt += 1;
    }

    for i in 0..cnt {
        a[l + i] = c[i];
    }
}

fn main() {
    input! {
        n: usize,
        mut a: [usize;n]
    }

    {
        let mut a_temp = A.lock().unwrap();
        a_temp.push(0);
        a_temp.append(&mut a);

        let mut c = C.lock().unwrap();
        for _ in 0..n {
            c.push(0);
        }
    }

    merge_sort(1, n + 1);
    
    println!("{}", C.lock().unwrap()[..n].iter().map(|x| x.to_string()).join(" "));
}
