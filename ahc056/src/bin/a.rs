#![allow(unused_imports, dead_code, non_snake_case)]
#![allow(unused_variables, unused_assignments)]
use itertools::{Itertools, iproduct};
use proconio::{derive_readable, fastout, input, marker::{Bytes, Chars, Isize1, Usize1}};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque}, fmt::{Display, Formatter, Result, format}, ops::{Index, IndexMut}};

const DIRECTIONS: [Direction;4] = [Direction::U, Direction::D, Direction::L, Direction::R];

#[derive_readable]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Pos {
    x: usize,
    y: usize
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    
    fn new_if_is_valid(&self, d: UnsafePos, n: usize) -> Option<Self> {
        let next = UnsafePos::new(self.x as isize + d.x, self.y as isize + d.y);
        if next.is_valid(n as isize) {
            Some(Self::new(next.x as usize, next.y as usize))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct UnsafePos {
    x: isize,
    y: isize
}

impl UnsafePos {
    const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    
    fn is_valid(&self, n: isize) -> bool {
        0 <= self.x && self.x < n && 0 <= self.y && self.y < n
    }
}

struct Grid<T> {
    g: Vec<Vec<T>>
}

impl<T: Clone> Grid<T> {
    fn new(n: usize) -> Self {
        Self { g: vec![vec![];n] }
    }

    fn new_with_default(n: usize, d: T) -> Self {
        Self { g: vec![vec![d;n];n] }
    }

    fn iter(&self) -> impl Iterator<Item = &Vec<T>> {
        self.g.iter()
    }

    fn pos_iter(&self) -> impl Iterator<Item = (Pos, &T)> {
        self.iter().enumerate().flat_map(|(i, row)| {
            row.iter().enumerate().map(move |(j, v)| (Pos::new(i, j), v))
        })
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.g[index]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.g[index]
    }
}

impl<T> Index<Pos> for Grid<T> {
    type Output = T;

    fn index(&self, index: Pos) -> &Self::Output {
        &self[index.x][index.y]
    }
}

impl<T> IndexMut<Pos> for Grid<T> {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self[index.x][index.y]
    }
}

struct Encoder {
    x: usize,
    y: usize,
    map: Vec<((usize, usize), (usize, usize))>
}

impl Encoder {
    fn new() -> Self {
        Self { x: 0, y: 0, map: vec![] }
    }
    
    fn get(&self, x: usize, y: usize) -> (usize, usize) {
        self.map.iter().find(|&(p, _)| (x, y).eq(p)).unwrap().1
    }
    
    fn encode(&mut self, x: usize, y: usize) -> (usize, usize) {
        let tmp = (self.x, self.y);
        self.map.push(((x, y), tmp));
        
        if self.x == self.y {
            self.x += 1;
            self.y = 0;
        } else if self.x == self.y + 1 {
            self.y += 1;
            self.x = 0;
        } else if self.x > self.y {
            self.y += 1;
        } else {
            self.x += 1;
        }
        
        tmp
    }
    
    fn decode(&self, x: usize, y: usize) -> (usize, usize) {
        self.map.iter().find(|&(_, n)| (x, y).eq(n)).unwrap().0
    }
}

trait VecUtil<T> {
    fn get_or_default(&self, i: usize) -> T;
}

impl<T: Default + Copy> VecUtil<T> for Vec<T> {
    fn get_or_default(self: &Vec<T>, i: usize) -> T {
        self.get(i).copied().unwrap_or_default()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Default)]
enum Direction {
    #[default]
    U,
    D,
    L,
    R,
    S,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let v = match self {
            Self::U => "U",
            Self::D => "D",
            Self::L => "L",
            Self::R => "R",
            Self::S => "S"
        };

        write!(f, "{}", v)?;
        Ok(())
    }
}

impl Direction {
    fn to_unsafe_pos(&self) -> UnsafePos {
        match self {
            Self::U => UnsafePos::new(-1, 0),
            Self::D => UnsafePos::new(1, 0),
            Self::L => UnsafePos::new(0, -1),
            Self::R => UnsafePos::new(0, 1),
            Self::S => UnsafePos::new(0, 0),
        }
    }
}

#[fastout]
fn main() {
    input! {
        (n, k, t): (usize, usize, usize),
        u: [Chars;n],
        h: [Chars;n-1],
        init: Pos,
        d: [Pos;k-1]
    }

    let mut g = Grid::new_with_default(n, vec![]);
    for (i, j, d) in iproduct!(0..n, 0..n, DIRECTIONS) {
        if let Some(p) = Pos::new(i, j).new_if_is_valid(d.to_unsafe_pos(), n)
            .filter(|&p| (i == p.x && '0'.eq(&u[i][j.min(p.y)])) || (j == p.y && '0'.eq(&h[i.min(p.x)][j])))
        {
            g[i][j].push(p);
        }
    }

    let mut aset = FxHashSet::default();
    for (p, v) in g.pos_iter() {
        if v.len() < 3 {
            let mut f = None;
            for i in DIRECTIONS {
                if let Some(p) = p.new_if_is_valid(i.to_unsafe_pos(), n).filter(|x| !v.contains(x)) {
                    aset.insert(p);

                    if let Some(f) = f {
                        aset.insert(p.new_if_is_valid(f, n).unwrap());
                    } else {
                        f.replace(i.to_unsafe_pos());
                    }
                }
            }
        }
    }

    let mut recs = vec![];
    for &i in &aset {
        if i.eq(&Pos::new(n-1, n-1)) || (i.x == n-1 && !g[i.x-1][i.y].contains(&i)) || (i.y == n-1 && !g[i.x][i.y-1].contains(&i)) {
            continue;
        }

        let mut x = Pos::default();
        for j in i.x+1..n {
            let p = Pos::new(j, i.y);
            if p.new_if_is_valid(Direction::R.to_unsafe_pos(), n).is_none_or(|x| aset.contains(&x) || !g[p].contains(&x)) {
                x = p;
            }
        }

        let mut y = Pos::default();
        for j in i.y+1..n {
            let p = Pos::new(i.x, j);
            if p.new_if_is_valid(Direction::D.to_unsafe_pos(), n).is_none_or(|x| aset.contains(&x) || !g[p].contains(&x)) {
                y = p;
            }
        }

        recs.push(vec![i, x, y, Pos::new(x.x, y.y)]);
    }
    
    let mut now = init;
    let mut m = Grid::new_with_default(n, vec![]);
    let mut routes = vec![];
    for (i, &v) in d.iter().enumerate() {
        let mut dist = Grid::new_with_default(n, !0usize);
        let mut q = VecDeque::new();
        q.push_back(v);
        dist[v] = 0;
        
        while let Some(p) = q.pop_front() {
            for &next in g[p].iter() {
                if dist[next] == !0 {
                    dist[next] = dist[p] + 1;
                    q.push_back(next);
                }
            }
        }
        
        let mut cnt = 0;
        while now != v {
            let mut min = Direction::default();
            let mut min_p = now;
            let mut min_c = !0;
            for i in DIRECTIONS {
                if let Some(p) = now.new_if_is_valid(i.to_unsafe_pos(), n).filter(|&p| g[now].contains(&p) && min_c > dist[p]) {
                    min = i;
                    min_p = p;
                    min_c = dist[p];
                }
            }
            m[now].push((min, i, cnt));
            
            cnt += 1;
            now = min_p;
        }
        
        routes.push(cnt-1);
    }
    
    let mut encoder = Encoder::new();
    let mut m2 = Grid::new_with_default(n, vec![]);
    let mut ans_m = Grid::new_with_default(n, None);
    for &(d, f, l) in &m[init] {
        let (c, q) = encoder.encode(f, l);
        m2[init].push((c, q, d));
        ans_m[init].get_or_insert(c);
    }

    for (p, v) in m.pos_iter() {
        if init.eq(&p) {
            continue;
        }

        for &(d, f, l) in v {
            let (c, q) = encoder.encode(f, l);
            m2[p].push((c, q, d));
            ans_m[p].get_or_insert(c);
        }
    }

    let mut ans = vec![];
    let mut ans_c = 0;
    let mut ans_q = 0;
    for (p, v) in m2.pos_iter() {
        for (k, &(c, q, d)) in v.iter().enumerate() {
            ans_c = ans_c.max(c);
            ans_q = ans_q.max(q);

            let next = p.new_if_is_valid(d.to_unsafe_pos(), n).unwrap();
            let (f, l) = encoder.decode(c, q);
            let i = if l == routes[f] {
                m[next].iter().position(|&(_, i, cnt)| (f+1, 0).eq(&(i, cnt))).unwrap_or_default()
            } else {
                m[next].iter().position(|&(_, i, cnt)| (f, l+1).eq(&(i, cnt))).unwrap_or_default()
            };

            ans.push(format!("{} {} {} {} {}", c, q, v.get_or_default(k+1).0, m2[next].get_or_default(i).1, d));
        }
    }

    println!("{} {} {}", ans_c+1, ans_q+1, ans.len());
    println!("{}", ans_m.iter().map(|x| x.iter().map(|x| x.unwrap_or_default()).join(" ")).join("\n"));
    println!("{}", ans.join("\n"));
}
