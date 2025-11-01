#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}, source::line::LineSource};
use rand::{rngs::StdRng, Rng};
use std::{collections::VecDeque, io::{stdin, stdout, BufReader, Write}};

const NEXTS: [UnsafePos;8] = [
    UnsafePos::new(1, 0), UnsafePos::new(-1, 0), UnsafePos::new(0, 1), UnsafePos::new(0, -1),
    UnsafePos::new(1, 1), UnsafePos::new(1, -1), UnsafePos::new(-1, 1), UnsafePos::new(-1, -1),
];

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pos {
    x: usize,
    y: usize
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct UnsafePos {
    x: isize,
    y: isize
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: Pos) -> usize {
        ((self.x as isize - other.x as isize).abs() + (self.y as isize - other.y as isize).abs()) as usize
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

impl UnsafePos {
    const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn is_valid(&self, n: isize) -> bool {
        0 <= self.x && self.x < n && 0 <= self.y && self.y < n
    }
}

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        n: usize,
        t: (usize, usize),
        b: [Chars; n],
    }

    let t = Pos::new(t.0, t.1);
    
    let mut revealed = vec![vec![false;n];n];
    let mut m = vec![vec![true;n];n];
    for (i, v) in b.iter().enumerate() {
        for (j, w) in v.iter().enumerate() {
            if 'T'.eq(w) {
                m[i][j] = false;
            }
        } 
    }
    m[t.x][t.y] = false;
    
    let mut places = 0;
    bfs(Pos::new(0, n/2), t, n, &m, &mut vec![vec![!0;n];n], &mut places);

    #[allow(unused_mut, unused_assignments)]
    let mut seed = rand::thread_rng().gen();

    #[cfg(feature = "seed")]
    {
        seed = 11216848234635351618;
    }

    let mut rnd: StdRng = rand::SeedableRng::seed_from_u64(seed);

    loop {
        input! {
            from &mut source,
            p: (usize, usize),
            num_coords: usize,
            xy: [usize;num_coords*2],
        }

        let p = Pos::new(p.0, p.1);

        if t.eq(&p) {
            break;
        }
        
        for k in 0..num_coords {
            revealed[xy[2*k]][xy[2*k+1]] = true;
        }

        let mut flag = true;
        let mut set = vec![];

        let mut dist = vec![vec![!0;n];n];
        let mut min_td = !0;
        let mut max_td = 0;
        let mut min_tp = NEXTS[0];
        let mut max_tp = NEXTS[0];

        bfs(t, t, n, &m, &mut dist, &mut 0);
        for i in NEXTS[..4].iter() {
            if let Some(next) = p.new_if_is_valid(*i, n) {
                if dist[next.x][next.y] == !0 || revealed[next.x][next.y] || !m[next.x][next.y] {
                    continue;
                }

                if dist[next.x][next.y] < min_td {
                    min_td = dist[next.x][next.y];
                    min_tp = *i;
                }
                
                if dist[next.x][next.y] > max_td {
                    max_td = dist[next.x][next.y];
                    max_tp = *i;
                }
            }
        }
        
        if min_td <= 10 {
            if let Some(nextp) = p.new_if_is_valid(max_tp, n) {
                flag = false;
                set.append(&mut find_edge(p, t, p, min_tp, n, &mut m, &revealed, &mut places, 1, 1));
    
                let mut dist = vec![vec![!0;n];n];
                m[p.x][p.y] = false;
                bfs(nextp, t, n, &m, &mut dist, &mut 0);
                let mut max = 0;
                let mut maxp = (p.x, p.y);
                for (x, i) in dist.iter().enumerate() {
                    for (y, &v) in i.iter().enumerate() {
                        if v != !0 && max < v {
                            maxp = (x, y);
                            max = v;
                        }
                    }
                }
    
                if maxp.0 != p.x || maxp.1 != p.y {
                    let maxp = Pos::new(maxp.0, maxp.1);
                    let mut now = nextp;
                    while let Some(next) = now.new_if_is_valid(max_tp, n) {
                        if !m[next.x][next.y] {
                            break;
                        }
                        
                        m[next.x][next.y] = false;
                        if !revealed[next.x][next.y] {
                            let mut dist = vec![vec![!0;n];n];
                            bfs(nextp, t, n, &m, &mut dist, &mut 0);
    
                            if dist[maxp.x][maxp.y] != !0 {
                                m[p.x][p.y] = true;
                                if can_it_reach(p, t, t, n, &m, &mut places, 1) {
                                    set.push(next);
                                    break;
                                }
                            }
                        }
                        m[next.x][next.y] = true;
                        now = next;
                    }
                }
    
                m[p.x][p.y] = true;
                for j in NEXTS[..4].iter() {
                    if !max_tp.eq(j) && !min_tp.eq(j) {
                        set.append(&mut find_edge(p, t, p, *j, n, &mut m, &revealed, &mut places, 2, 1));
                    }
                }
            }
        }

        let mut dist = vec![vec![!0;n];n];
        let mut min_pd = !0;
        let mut min_pp = NEXTS[0];

        bfs(p, t, n, &m, &mut dist, &mut 0);
        for i in NEXTS.iter() {
            if let Some(next) = t.new_if_is_valid(*i, n) {
                if dist[next.x][next.y] != !0 && dist[next.x][next.y] < min_pd && !revealed[next.x][next.y] && m[next.x][next.y] {
                    min_pd = dist[next.x][next.y];
                    min_pp = *i;
                }
            }
        }
        
        if min_pd <= 5 && p.new_if_is_valid(min_pp, n).is_some() {
            flag = false;

            if min_pp.x == 0 || min_pp.y == 0 {
                set.append(&mut find_edge(p, t, t, min_pp, n, &mut m, &revealed, &mut places, 1, n.pow(2)/100));
            } else {
                set.append(&mut find_edge(p, t, t, NEXTS.iter().copied().find(|p| p.x == 0 && p.y == min_pp.y).unwrap(), n, &mut m, &revealed, &mut places, 1, n.pow(2)/100));
                set.append(&mut find_edge(p, t, t, NEXTS.iter().copied().find(|p| p.x == min_pp.x && p.y == 0).unwrap(), n, &mut m, &revealed, &mut places, 1, n.pow(2)/100));
            }
        } 
        
        if flag {
            if rand_chance(&mut rnd, n, 30) {
                let mut d = None;
                for i in NEXTS[..4].iter() {
                    if let Some(next) = p.new_if_is_valid(*i, n) {
                        if m[next.x][next.y] {
                            if let Some(_) = next.new_if_is_valid(*i, n) {
                                d = Some(*i);
                            }
                        }
                    }
                }

                if let Some(d) = d {
                    set.append(&mut find_edge(p, t, p, d, n, &mut m, &revealed, &mut places, 2, 1));
                    let ne = p.new_if_is_valid(d, n).unwrap().new_if_is_valid(d, n).unwrap();
                    if d.x == 0 {
                        set.append(&mut find_edge(p, t, ne, UnsafePos::new(1, 0), n, &mut m, &revealed, &mut places, 1, 1));
                        set.append(&mut find_edge(p, t, ne, UnsafePos::new(-1, 0), n, &mut m, &revealed, &mut places, 1, 1));
                    } else {
                        set.append(&mut find_edge(p, t, ne, UnsafePos::new(0, 1), n, &mut m, &revealed, &mut places, 1, 1));
                        set.append(&mut find_edge(p, t, ne, UnsafePos::new(0, -1), n, &mut m, &revealed, &mut places, 1, 1));
                    }
                }
            }
        
            for i in NEXTS[..4].iter() {
                if rand_chance(&mut rnd, n, 200) {
                    set.append(&mut find_edge(p, t, p, *i, n, &mut m, &revealed, &mut places, 3, 1));
                } else {
                    set.append(&mut find_edge(p, t, p, *i, n, &mut m, &revealed, &mut places, 2, 1));
                }
            }
        }
        
        print!("{} ", set.len());
        for i in set {
            print!("{} {} ", i.x, i.y);
        }
        println!();
        stdout().flush().unwrap();

    }
}

fn bfs(p: Pos, t: Pos, n: usize, m: &Vec<Vec<bool>>, dist: &mut Vec<Vec<usize>>, ans: &mut usize) {
    let mut q = VecDeque::new();
    q.push_back(p);
    *ans = 1;
    dist[p.x][p.y] = 0;
    
    while let Some(pos) = q.pop_front() {
        for i in NEXTS[..4].iter() {
            if let Some(next) = pos.new_if_is_valid(*i, n) {
                if dist[next.x][next.y] == !0 && m[next.x][next.y] {
                    *ans += 1;
                    let mut cnt = 0;
                    for j in NEXTS[..4].iter() {
                        if let Some(next) = next.new_if_is_valid(*j, n) {
                            if m[next.x][next.y] {
                                cnt += 1;
                            }
                        }
                    }

                    if cnt <= 2 {
                        dist[next.x][next.y] = dist[pos.x][pos.y];
                        if pos.eq(&t) || next.distance(t) >= 2 {
                            q.push_back(next);
                        } else {
                            dist[t.x][t.y] = dist[pos.x][pos.y] + 1;
                        }
                    } else {
                        dist[next.x][next.y] = dist[pos.x][pos.y] + 1;
                        if pos.eq(&t) || next.distance(t) >= 2 {
                            q.push_back(next);
                        } else {
                            dist[t.x][t.y] = dist[pos.x][pos.y] + 2;
                        }
                    }
                }
            }
        }
    }
}

fn can_it_reach(p: Pos, t: Pos, g: Pos, n: usize, m: &Vec<Vec<bool>>, places: &mut usize, loss: usize) -> bool {
    let mut dist = vec![vec![!0;n];n];
    let mut ans = 0;
    bfs(p, t, n, m, &mut dist, &mut ans);

    if dist[g.x][g.y] != !0 && ans+loss >= *places {
        *places = ans;
        true
    } else {
        false
    }
}

fn find_edge(p: Pos, t: Pos, mut origin: Pos, i: UnsafePos, n: usize, m: &mut Vec<Vec<bool>>, revealed: &Vec<Vec<bool>>, places: &mut usize, mut d: usize, loss: usize) -> Vec<Pos> {
    loop {
        if d == 1 {
            while let Some(next) = origin.new_if_is_valid(i, n) {
                if !m[next.x][next.y] {
                    return vec![];
                }

                m[next.x][next.y] = false;
                if !revealed[next.x][next.y] && can_it_reach(p, t, t, n, &m, places, loss) {
                    return vec![next];
                } else {
                    m[next.x][next.y] = true;
                    origin = next;
                }
            }
        }

        if let Some(next) = origin.new_if_is_valid(i, n) {
            if !m[next.x][next.y] {
                return vec![];
            }

            origin = next;
            d -= 1;
        } else {
            return vec![];
        }
    }
}

fn rand_chance(rnd: &mut StdRng, n: usize, base: usize) -> bool {
    let scale = if n <= 25 { 2 } else { 1 };
    (*rnd).gen_range(0..(base * scale)) == 0
}