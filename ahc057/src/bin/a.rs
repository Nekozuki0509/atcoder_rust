#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use itertools::Itertools;
use proconio::{derive_readable, fastout, input, marker::{Bytes, Chars, Isize1, Usize1}};
use std::{collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque}, mem::swap};

#[derive_readable]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Pos {
    x: f64,
    y: f64
}

impl Pos {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn to(&self, v: Pos, l: usize, time: usize) -> Self {
        Self::new((self.x + v.x * time as f64).rem_euclid(l as f64), (self.y + v.y * time as f64).rem_euclid(l as f64))
    }

    fn cost(&self, other: Self, l: usize) -> usize {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        let mx = dx.min(l as f64 - dx);
        let my = dy.min(l as f64 - dy);
        (mx * mx + my * my).sqrt().round() as usize
    }
}

struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self { par: (0..n).collect(), siz: vec![1; n] }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }

        self.par[x] = self.root(self.par[x]);
        self.par[x]
    }

    fn unite(&mut self, mut parent: usize, mut child: usize) -> usize {
        parent = self.root(parent);
        child = self.root(child);
        
        if parent == child {
            return parent;
        }

        if self.siz[parent] < self.siz[child] {
            swap(&mut parent, &mut child);
        }

        self.par[child] = parent;
        self.siz[parent] += self.siz[child];

        parent
    }

    fn is_same(&mut self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}

#[fastout]
fn main() {
    input! {
        (n, _, _, _, l): (usize, usize, usize, usize, usize),
        d: [(Pos, Pos);n]
    }

    let mut uf = UnionFind::new(n);
    let (mut m, mut v): (Vec<Pos>, Vec<Pos>) = d.iter().copied().unzip();
    for (b, (&(p, _), &(k, c))) in [(0, 1), (517, 150), (758, 70), (861, 30), (895, 10)].iter().tuple_windows().enumerate() {
        let mut costs = BTreeMap::new();
        let mut tm = m.clone();
        for t in p..k {
            for (i, j) in (0..n).tuple_combinations() {
                if !uf.is_same(i, j) && uf.size(i) == (1 << b) && uf.size(j) == (1 << b) {
                    let mut ri = uf.root(i);
                    let mut rj = uf.root(j);
                    if ri > rj {
                        swap(&mut ri, &mut rj);
                    }

                    let now = *costs.get(&(ri, rj)).unwrap_or(&(!0, 0, 0, 0, Pos::default(), Pos::default()));
                    let cost = tm[i].cost(tm[j], l);
                    if now.0 > cost {
                        // dbg!(t);
                        costs.insert((ri, rj), (cost, t, i, j, tm[i], tm[j]));
                    }
                }
            }
    
            tm = tm.iter().copied().enumerate().map(|(i, p)| p.to(v[uf.root(i)], l, 1)).collect();
        }

        let mut f = vec![false;n];
        let mut cnt = 0;
        for (&(ri, rj), &(_, t, i, j, ti, tj)) in costs.iter().sorted_by_key(|&(_, &(cost, _, _, _, _, _))| cost) {
            if f[ri] || f[rj] {
                continue;
            }

            println!("{} {} {}", t, i, j);
            let si = uf.size(i) as f64;
            let sj = uf.size(j) as f64;
            let nv = Pos::new((si * v[i].x + sj * v[j].x) / (si + sj), (si * v[i].y + sj * v[j].y) / (si + sj));
            v[uf.unite(i, j)] = nv;

            m[i] = ti.to(nv, l, k-t);
            m[j] = tj.to(nv, l, k-t);

            f[ri] = true;
            f[rj] = true;

            cnt += 1;
            if cnt == c {
                break;
            }
        }

        f.iter().enumerate().for_each(|(i, &b)| if !b { m[i] = m[i].to(v[i], l, k-l) });
    }

    let mut costs = BTreeMap::new();
    let mut tm = m.clone();
    for t in 895..1000 {
        for (i, j) in (0..n).tuple_combinations() {
            if !uf.is_same(i, j) && (uf.size(i) == 16 && uf.size(j) != 16) || (uf.size(i) != 16 && uf.size(j) == 16) {
                let mut ri = uf.root(i);
                let mut rj = uf.root(j);
                if ri > rj {
                    swap(&mut ri, &mut rj);
                }

                let now = *costs.get(&(ri, rj)).unwrap_or(&(!0, 0, 0, 0));
                let cost = tm[i].cost(tm[j], l);
                if now.0 > cost {
                    costs.insert((ri, rj), (cost, t, i, j));
                }
            }
        }

        tm = tm.iter().copied().enumerate().map(|(i, p)| p.to(v[uf.root(i)], l, 1)).collect();
    }

    for (c, (&_, &(_, t, i, j))) in costs.iter().sorted_by_key(|&(_, &(cost, _, _, _))| cost).enumerate() {
        if uf.size(i) & uf.size(j) != 0 {
            continue;
        }

        println!("{} {} {}", t, i, j);

        uf.unite(i, j);
    }
}
