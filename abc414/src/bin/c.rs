use itertools::Itertools;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        (a, n): (usize, usize)
    }

    let mut an = 0usize;
    for i in 1usize..1_000_000 {

        let mut l = i.to_string();
        let mut r = i.to_string().chars().rev().collect::<String>();
        let mut f = l.clone();

        f.push_str(&r);
        let tn = f.parse::<usize>().unwrap();
        //dbg!(tn);
        if tn > n {
            break;
        }
        
        {
            let nn = shinsu(tn, a).to_string().chars().collect_vec();
            //dbg!(&nn);
            let ni = nn.len() / 2;
            let mut l = nn[..=ni-1].to_vec();
            let mut r = nn[nn.len()-ni..].to_vec();
            r.reverse();
            if l.eq(&r) {
                an += tn;
            }
        }

        for j in 0..10 {
            let mut nl = l.clone();
            nl.push_str(&i.to_string());
            nl.push_str(&r);
            let ntn = shinsu(nl.parse().unwrap(), a);
            let nt = ntn.to_string().chars().collect_vec();
            let ni = nt.len() / 2;
            let mut l = nt[..=ni-1].to_vec();
            let mut r = nt[nt.len()-ni..].to_vec();
            r.reverse();
            if l.eq(&r) {
                an += tn;
            }
        }
    }

    println!("{}", an);
}

fn shinsu(mut x: usize,b: usize)->usize{
    let mut amari: Vec<usize> = Vec::new();
    while x != 0{
        amari.push(x % b);
        x /= b;
    }
    let mut n:usize = 0;
    for i in 0..amari.len(){
        n += 10usize.pow(i as u32)*amari[i]
    }
    return n;
}
