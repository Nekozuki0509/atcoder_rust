use std::collections::{BTreeSet, VecDeque};

use indexmap::IndexMap;
use itertools::Itertools;
use num::abs;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Place {
    i: usize,
    point: Point
}

struct Sorter {
    i: usize,
    place: Place,
    main: isize,
    sub: isize,
    out: u8
}

struct Processor {
    i: usize,
    place: Place
}

impl Point {
    fn new(p: (i64, i64)) -> Self {
        Self {x: p.0, y: p.1}
    }

    fn check_intersection(self: &Self, to: &Point, belts: &Vec<(Point, Point)>) -> bool {
        fn sign(x: i64) -> i64 {
            if x > 0 { 1 } else if x < 0 { -1 } else { 0 }
        }

        fn orientation(a: Point, b: Point, c: Point) -> i64 {
            let cross = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
            sign(cross)
        }

        fn segments_intersect(p1: Point, p2: Point, q1: Point, q2: Point) -> bool {
            if  p1.x.max(p2.x) < q1.x.min(q2.x) ||
                q1.x.max(q2.x) < p1.x.min(p2.x) ||
                p1.y.max(p2.y) < q1.y.min(q2.y) ||
                q1.y.max(q2.y) < p1.y.min(p2.y)  {
                return false;
            }
            let o1 = orientation(p1, p2, q1);
            let o2 = orientation(p1, p2, q2);
            let o3 = orientation(q1, q2, p1);
            let o4 = orientation(q1, q2, p2);
            (o1 * o2 <= 0) && (o3 * o4 <= 0)
        }

        for &(belt_start, belt_end) in belts {
            // 端点を共有する場合は交差とみなさない
            if  (self.x == belt_start.x && self.y == belt_start.y) ||
                (self.x == belt_end.x && self.y == belt_end.y) ||
                (to.x == belt_start.x && to.y == belt_start.y) ||
                (to.x == belt_end.x && to.y == belt_end.y) {
                continue;
            }
            if segments_intersect(*self,*to, belt_start, belt_end) {
                return true;
            }
        }
        false
    }

    fn get_distance(self: &Self, to: &Point) -> u64 {
        ((self.x - to.x).abs() as u64).pow(2) + ((self.y - to.y).abs() as u64).pow(2)
    }
}

impl Place {
    fn new(i: usize, point: Point) -> Self {
        Self { i, point }
    }
}

impl Sorter {
    fn new(i: usize, place: Place, main: isize, sub: isize, out: u8) -> Self {
        Self { i, place, main, sub, out }
    }
}

impl Processor {
    fn new(i: usize, place: Place) -> Self {
        Self { i, place }
    }
}

fn find_largest_prob_difference(w: &Vec<usize>, prob: &Vec<Vec<f64>>) -> (usize, Vec<usize>, Vec<usize>, i32, u8) {
    let prob = prob.iter().enumerate()
        .map(|(i, v)| (
            i, v.iter().enumerate()
                .filter(|(i, _)| w.contains(&i))
                .sorted_by(|(_, j1), (_, j2)| j1.partial_cmp(j2).unwrap())
                .collect::<Vec<_>>()
            )
        )
        .collect::<IndexMap<usize, Vec<_>>>();

    let r = prob.iter()
        .filter_map(|(i, v)| {
            match v.iter().position(|&(_, &p)| 0.5 <= p ) {
                Some(x) => if x != 0 {Some((i, (x, v[x].1-v[x-1].1)))} else {None},
                None => None
            }
        })
        .max_by(|&(_, (_, p1)), (_, (_, p2))| p1.partial_cmp(p2).unwrap())
        .map(|(i, (j, _))| (i, j));

    if let Some((&sorter, waste)) = r {
        (sorter, prob[sorter][waste..].iter().map(|&(i, _)| i).collect(), prob[sorter][..waste].iter().map(|&(i, _)| i).collect(), 1, 1)
    } else {
        //(sorter, relative_waste_i, max_waste_prob_difference, max_waste_prob)
        let mut max = (0, 0, 0., 0.);
        for (i, v) in &prob {
            for (j, &(_, w)) in v.iter().enumerate() {
                if j == 0 {
                    continue;
                }
    
                if max.2 < w-v[j-1].1 || (max.2 == w-v[j-1].1 && max.3 < (w.max(1.-w)).max(v[j-1].1.max(1.-v[j-1].1))) {
                    max = (*i, j, w-v[j-1].1, (w.max(1.-w)).max(v[j-1].1.max(1.-v[j-1].1)));
                }
            }
    
        }
    
        (max.0, prob[max.0][max.1..].iter().map(|&(i, _)| i).collect(), prob[max.0][..max.1].iter().map(|&(i, _)| i).collect(), 1, 1)
        
    }
}

fn find_largest_prob_difference1(w: &Vec<usize>, prob: &Vec<Vec<f64>>) -> (usize, Vec<usize>, Vec<usize>, i32, u8) {
    let prob = prob.iter().enumerate()
        .map(|(i, v)| (
            i, v.iter().enumerate()
                .filter(|(i, _)| w.contains(&i))
                .sorted_by(|(_, j1), (_, j2)| j1.partial_cmp(j2).unwrap())
                .collect::<Vec<_>>()
            )
        )
        .collect::<IndexMap<usize, Vec<_>>>();

    //(sorter, relative_waste_i, counter, max_waste_prob_difference, efficiency)
    let mut out = 1u8;
    let mut max = None;
    for (i, v) in &prob {
        let mut probs = v.clone();
        if *probs.iter().max_by(|(_, &p1), (_, p2)| p1.partial_cmp(p2).unwrap()).unwrap().1 < 0.5 {
            probs = probs.iter().map(|(_, &p)| 1.-p).collect();
            out = 2;
        }

        while *probs.iter().max_by(|(_, &p1), (_, p2)| p1.partial_cmp(p2).unwrap()).unwrap().1 < 0.5 {
            let efficiency = probs.iter().map(|(_, &p)| (p-0.5).powf(2.0)).sum() / probs.len() as f64;
            let mut counter = 1;
            if let Some(x) = probs.iter().position(|&(_, &p)| p > 0.5) {
                if max.is_none() || (x != 0 && (max.unwrap().3 < probs[x].1-probs[x-1].1 || (max.unwrap().3 == probs[x].1-probs[x-1].1 && max.unwrap().4 < efficiency || max.unwrap().4 == efficiency))) {
                    max = Some((*i, x, counter, probs[x].1-probs[x-1].1, efficiency));
                }
            }
            probs = probs.iter().map(|&(j, &p)| (j, p * prob[i][j])).collect();
            counter += 1;
        }
    }

    if let Some((sorter, waste, counter, _, _)) = max {
        (sorter, prob[sorter][waste..].iter().map(|&(i, _)| i).collect(), prob[sorter][..waste].iter().map(|&(i, _)| i).collect(), counter, out)
    } else {
        let mut max = (0, 0, 0., 0.);
        for (i, v) in &prob {
            for (j, &(_, w)) in v.iter().enumerate() {
                if j == 0 {
                    continue;
                }

                if max.2 < w-v[j-1].1 || (max.2 == w-v[j-1].1 && max.3 < (w.max(1.-w)).max(v[j-1].1.max(1.-v[j-1].1))) {
                    max = (*i, j, w-v[j-1].1, (w.max(1.-w)).max(v[j-1].1.max(1.-v[j-1].1)));
                }
            }

        }

        (max.0, prob[max.0][max.1..].iter().map(|&(i, _)| i).collect(), prob[max.0][..max.1].iter().map(|&(i, _)| i).collect(), 1, 1)

    }
}

fn get_better(r1: Option<(i64, String, Vec<Sorter>, usize)>, r2: Option<(i64, String, Vec<Sorter>, usize)>) -> Option<(i64, String, Vec<Sorter>, usize)> {
    if r1.is_none() && r2.is_none() {
        None
    } else if r1.is_some() && r2.is_none() {
        r1
    } else if r1.is_none() && r2.is_some() {
        r2
    } else {
        let r1 = r1.unwrap();
        let r2 = r2.unwrap();
        if r1.0 < r2.0 {
            Some(r1)
        } else {
            Some(r2)
        }
    }
}

fn compute_score(n: usize, m: usize, prob: &Vec<Vec<f64>>, 
                ds: &Vec<usize>, s: usize, cs: &Vec<(usize, isize, isize)>) -> i64 {
    // トポロジカルソート用のグラフ構築
    let mut g = vec![vec![]; n + m];
    for i in 0..m {
        let (sorter_type, main, sub) = cs[i];
        if sorter_type != usize::MAX {
            if main >= 0 {
                g[n + i].push(main as usize);
            }
            if sub >= 0 {
                g[n + i].push(sub as usize);
            }
        }
    }
    
    // 簡易トポロジカルソート
    let mut in_deg = vec![0; n + m];
    for u in 0..n + m {
        for &v in &g[u] {
            in_deg[v] += 1;
        }
    }
    
    let mut queue = VecDeque::new();
    for u in 0..n + m {
        if in_deg[u] == 0 {
            queue.push_back(u);
        }
    }
    
    let mut order = Vec::new();
    while let Some(u) = queue.pop_front() {
        order.push(u);
        for &v in &g[u] {
            in_deg[v] -= 1;
            if in_deg[v] == 0 {
                queue.push_back(v);
            }
        }
    }
    
    // 確率計算
    let mut probs = vec![vec![0.0; n]; n + m];
    probs[s].fill(1.0);
    
    for u in order {
        if u >= n {
            let sorter_idx = u - n;
            let (sorter_type, main, sub) = cs[sorter_idx];
            if sorter_type == usize::MAX {
                continue;
            }
            
            for waste_type in 0..n {
                let current_prob = probs[u][waste_type];
                if current_prob > 0.0 {
                    let p1 = prob[sorter_type][waste_type];
                    let p2 = 1.0 - p1;
                    
                    if main >= 0 {
                        probs[main as usize][waste_type] += current_prob * p1;
                    }
                    if sub >= 0 {
                        probs[sub as usize][waste_type] += current_prob * p2;
                    }
                }
            }
        }
    }
    
    // スコア計算
    let mut total_failure = 0.0;
    for i in 0..n {
        //dbg!(probs.len(), i, probs[i].len(), ds[i]);
        let success_prob = probs[i][ds[i]];
        total_failure += 1.0 - success_prob;
    }

    //eprintln!("{}", (1e9 * total_failure / n as f64).round() as i64);
    
    (1e9 * total_failure / n as f64).round() as i64
}

fn t1_solve(n: usize, m: usize, k: usize, mut processor_positions: Vec<(i64, i64)>, sorter_positions: Vec<(i64, i64)>, prob: Vec<Vec<f64>>) -> Option<(i64, String, Vec<Sorter>, usize)> {
    fn solve(n: usize, m: usize, _k: usize, mut sorter_positions: Vec<Place>, processor_positions: Vec<Place>, prob: Vec<Vec<f64>>, t: usize, pt: u8,
                find_any_position: fn(usize, usize, &Vec<Point>, &Vec<usize>, &Vec<Sorter>, &Vec<Place>, &Vec<Place>, &Vec<Place>, &Vec<(Point, Point)>) -> Option<(u8, Place)>,
                find_sorter_position: fn(usize, usize, usize, &Vec<Point>, &mut Vec<Place>, &Vec<Place>, &mut Vec<Place>, &mut Vec<(Point, Point)>) -> Option<Place>,
                find_largest_prob_difference: fn(&Vec<usize>, &Vec<Vec<f64>>) -> (usize, Vec<usize>, Vec<usize>, i32, u8)
            ) -> Option<(i64, String, Vec<Sorter>, usize)> {
        let mut inlet_conn = 0;
        let mut belts = vec![];
        let mut used_sorter_positions = vec![];
        let mut used_processor_positions = vec![];
        let mut fins: Vec<Sorter> = vec![];
        let mut finp = vec![];
    
        let mut q = VecDeque::new();
        q.push_back(((0..n).collect::<Vec<usize>>(), 0, n-1, vec![-1isize], vec![], 0usize));
    
        while !q.is_empty() {
            let (w, aleft, aright, mut froms, mut routes, max) = q.pop_front().unwrap();
            if w.len() == 1 {
                let r = find_any_position(aleft, aright, &froms.iter().map(|&i| fins[i as usize].place.point).collect(), &routes, &fins, &processor_positions, &used_sorter_positions, &used_processor_positions, &belts);
                match r {
                    Some((ty, p)) => {
                        for from in froms {
                            if fins[from as usize].main == -1 {
                                fins[from as usize].main = p.i as isize;
                            } else {
                                fins[from as usize].sub = p.i as isize;
                            }
                            belts.push((fins[from as usize].place.point, p.point));
            
                            if ty == 0 {
                                finp.push(Processor::new(max, p));
                                used_processor_positions.push(p);
                            }
                        }
                    },
                    None => return None
                }
                
                continue;
            }
    
            let (sorter, left, right, counter, out) = find_largest_prob_difference(&w, &prob);
            let mut nfroms = vec![];
            let mut flag = true;
            for _ in 0..counter {
                let r = find_sorter_position(t, aleft, aright, if froms[0] != -1 {froms.iter().map(|&i| fins[i as usize].place.point).collect()} else {&vec![Point::new((0, 5000))]}, &mut sorter_positions, &processor_positions, &mut used_sorter_positions, &mut belts);
                match r {
                    Some(p) => {
                        fins.push(Sorter::new(sorter, p, -1, -1, out));
                        for from in froms {
                            match from {
                                -1 => {
                                    inlet_conn = p.i;
                                    belts.push((Point::new((0, 5000)), p.point));
                                },
                                _ => {
                                    if fins[from as usize].main == -1 {
                                        fins[from as usize].main = p.i as isize;
                                    } else {
                                        fins[from as usize].sub = p.i as isize;
                                    }
                                    belts.push((fins[from as usize].place.point, p.point));
                                }
                            }
                        }

                        froms = vec![*froms.last().unwrap()];
                        nfroms.push(p.i as isize);
                        routes.push(p.i);
                    },
        
                    None => {
                        flag = false;
                        let r = find_any_position(aleft, aright, &froms.iter().map(|&i| fins[i as usize].place.point).collect(), &routes, &fins, &processor_positions, &used_sorter_positions, &used_processor_positions, &belts);
                        match r {
                            Some((ty, p)) => {
                                for from in froms {
                                    match from {
                                        -1 => {
                                            inlet_conn = p.i;
                                            belts.push((Point::new((0, 5000)), p.point));
                                        },
                                        _ => {
                                            if fins[from as usize].main == -1 {
                                                fins[from as usize].main = p.i as isize;
                                            } else {
                                                fins[from as usize].sub = p.i as isize;
                                            }
                                            belts.push((fins[from as usize].place.point, p.point));
                                        }
                                    }
                                }
                
                                if ty == 0 {
                                    finp.push(Processor::new(max, p));
                                    used_processor_positions.push(p);
                                }
                            },
                            None => return None
                        }
                    }
                }
    
                if flag {
                    match pt {
                        0 => q.push_back((
                            left.clone(), aleft, aright-right.len(), froms.clone(), routes.clone(), prob[sorter]
                                .iter().enumerate()
                                .filter(|(i, _)| left.contains(i))
                                .max_by(|(_, p1), (_, p2)| p1.partial_cmp(p2).unwrap()).unwrap().0
                        )),
                        _ => q.push_front((
                            left.clone(), aleft, aright-right.len(), froms.clone(), routes.clone(), prob[sorter]
                                .iter().enumerate()
                                .filter(|(i, _)| left.contains(i))
                                .max_by(|(_, p1), (_, p2)| p1.partial_cmp(p2).unwrap()).unwrap().0
                        ))
                    }
                    q.push_front((
                        right.clone(), aleft+left.len(), aright, nfroms, routes.clone(), prob[sorter]
                            .iter().enumerate()
                            .filter(|(i, _)| right.contains(i))
                            .max_by(|(_, p1), (_, p2)| p1.partial_cmp(p2).unwrap()).unwrap().0
                    ));
                }
            }
        } 
    
        let mut pset = BTreeSet::new();
        let mut vset = BTreeSet::new();
        for i in 0..n {
            pset.insert(i);
            vset.insert(i);
        }
    
        let mut finp = finp.iter()
            .map(|p| (p.place.i, p.i))
            .collect::<Vec<_>>();
    
        for (i, v) in &finp {
            pset.remove(i);
            vset.remove(v);
        }
    
        for (i, v) in pset.iter().zip(vset.iter()) {
            finp.push((*i, *v));
        }
        
        let ds: Vec<usize> = finp.iter()
            .sorted_by_key(|&p| p.0)
            .map(|p| p.1)
            .collect();
        
        fins.sort_by_key(|s| s.place.i);
        let mut cs = vec![(usize::MAX, -1isize, -1isize); m];
        
        for s in &fins {
            let idx = s.place.i - n;
            cs[idx] = (s.i, s.sub, s.main);
        }
    
        let finp = ds.iter()
            .map(|&v| v.to_string())
            .join(" ");
    
        Some((compute_score(n, m, &prob, &ds, inlet_conn, &cs), finp, fins, inlet_conn))
    }
    
    fn find_any_position_distance(left: usize, right: usize, froms: &Vec<Point>, routes: &Vec<usize>, fins: &Vec<Sorter>, processor_positions: &Vec<Place>, used_sorter_positions: &Vec<Place>, used_processor_positions: &Vec<Place>, belts: &Vec<(Point, Point)>) -> Option<(u8, Place)> {
        processor_positions[left+1..=right+1].iter()
            .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)) && p.i != processor_positions.len()-2 && p.i != processor_positions.len()+1)
            .min_by_key(|p| abs((processor_positions[left+1].point.y+processor_positions[right+1].point.y)/2-p.point.y))
            .map(| &p| (0, p))
            .or_else(|| 
                used_sorter_positions.iter()
                    .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)) && !routes.contains(&p.i))
                    .max_by_key(|p| froms.iter().map(|&v| p.point.get_distance(&v)).sum::<usize>())
                    .map(| &p| (1, p))
            )
            .or_else(|| 
                used_processor_positions.iter()
                    .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
                    .min_by_key(|p| froms.iter().map(|&v| p.point.get_distance(&v)).sum::<usize>())
                    .map(| &p| (1, p))
            )
    }
    
    fn find_any_position_x(left: usize, right: usize, froms: &Vec<Point>, routes: &Vec<usize>, fins: &Vec<Sorter>, processor_positions: &Vec<Place>, used_sorter_positions: &Vec<Place>, used_processor_positions: &Vec<Place>, belts: &Vec<(Point, Point)>) -> Option<(u8, Place)> {
        processor_positions[left+1..=right+1].iter()
            .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)) && p.i != processor_positions.len()-2 && p.i != processor_positions.len()+1)
            .min_by_key(|p| abs((processor_positions[left+1].point.y+processor_positions[right+1].point.y)/2-p.point.y))
            .map(| &p| (0, p))
            .or_else(|| 
                used_sorter_positions.iter()
                    .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)) && !routes.contains(&p.i))
                    .max_by_key(|p| p.point.x)
                    .map(| &p| (1, p))
            )
            .or_else(|| 
                used_processor_positions.iter()
                    .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
                    .min_by_key(|p| p.point.x)
                    .map(| &p| (1, p))
            )
    }
    
    fn find_any_position_center_x(left: usize, right: usize, froms: &Vec<Point>, routes: &Vec<usize>, fins: &Vec<Sorter>, processor_positions: &Vec<Place>, used_sorter_positions: &Vec<Place>, used_processor_positions: &Vec<Place>, belts: &Vec<(Point, Point)>) -> Option<(u8, Place)> {
        processor_positions[left+1..=right+1].iter()
            .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)) && p.i != processor_positions.len()-2 && p.i != processor_positions.len()+1)
            .min_by_key(|p| abs((processor_positions[left+1].point.y+processor_positions[right+1].point.y)/2-p.point.y))
            .map(| &p| (0, p))
            .or_else(|| 
                used_sorter_positions.iter()
                    .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)) && !routes.contains(&p.i))
                    .max_by_key(|p| abs((processor_positions[left+1].point.y+processor_positions[right+1].point.y)/2-p.point.y) + p.point.x)
                    .map(| &p| (1, p))
            )
            .or_else(|| 
                used_processor_positions.iter()
                    .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
                    .min_by_key(|p| abs((processor_positions[left+1].point.y+processor_positions[right+1].point.y)/2-p.point.y))
                    .map(| &p| (1, p))
            )
    }
    
    fn find_any_position_center_xx(left: usize, right: usize, froms: &Vec<Point>, routes: &Vec<usize>, fins: &Vec<Sorter>, processor_positions: &Vec<Place>, used_sorter_positions: &Vec<Place>, used_processor_positions: &Vec<Place>, belts: &Vec<(Point, Point)>) -> Option<(u8, Place)> {
        processor_positions[left+1..=right+1].iter()
            .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)) && p.i != processor_positions.len()-2 && p.i != processor_positions.len()+1)
            .min_by_key(|p| abs((processor_positions[left+1].point.y+processor_positions[right+1].point.y)/2-p.point.y))
            .map(| &p| (0, p))
            .or_else(|| 
                used_sorter_positions.iter()
                    .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)) && !routes.contains(&p.i))
                    .max_by_key(|p| abs((processor_positions[left+1].point.y+processor_positions[right+1].point.y)/2-p.point.y) + froms.iter().map(|&v| abs(p.point.x - v.x)).sum::<usize>())
                    .map(| &p| (1, p))
            )
            .or_else(|| 
                used_processor_positions.iter()
                    .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
                    .min_by_key(|p| abs((processor_positions[left+1].point.y+processor_positions[right+1].point.y)/2-p.point.y) + froms.iter().map(|&v| abs(p.point.x - v.x)).sum::<usize>())
                    .map(| &p| (1, p))
            )
    }

    fn find_sorter_position(t: usize, left: usize, right: usize, froms: &Vec<Point>, sorter_positions: &mut Vec<Place>, processor_positions: &Vec<Place>, used_sorter_positions: &mut Vec<Place>, belts: &mut Vec<(Point, Point)>) -> Option<Place> {
        let r = sorter_positions
            .iter().enumerate()
            .filter(|&(_, p)| processor_positions[left+t].point.y < p.point.y && p.point.y < processor_positions[right+t].point.y && !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
            .min_by_key(|&(_, p)| p.point.x)
            .or_else(|| {
                sorter_positions
                    .iter().enumerate()
                    .filter(|&(_, p)| processor_positions[left+t].point.y <= p.point.y && p.point.y < processor_positions[right+t].point.y && !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
                    .min_by_key(|&(_, p)| p.point.x)
            })
            .or_else(|| {
                sorter_positions
                    .iter().enumerate()
                    .filter(|&(_, p)| processor_positions[left+t].point.y <= p.point.y && p.point.y <= processor_positions[right+t].point.y && !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
                    .min_by_key(|&(_, p)| p.point.x)
            })
            .or_else(|| {
                sorter_positions
                    .iter().enumerate()
                    .filter(|&(_, p)| p.point.y <= processor_positions[right+t].point.y && !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
                    .min_by_key(|&(_, p)| p.point.x)
            });

        match r {
            Some((i, &p)) => {
                used_sorter_positions.push(sorter_positions.remove(i));
                for &from in froms {
                    belts.push((from, p.point));
                }
                Some(p)
            },
            None => None
        }
    }

    fn find_sorter_position1(t: usize, left: usize, right: usize, froms: &Vec<Point>, sorter_positions: &mut Vec<Place>, processor_positions: &Vec<Place>, used_sorter_positions: &mut Vec<Place>, belts: &mut Vec<(Point, Point)>) -> Option<Place> {
        let r = sorter_positions
            .iter().enumerate()
            .filter(|&(_, p)| processor_positions[left+t].point.y < p.point.y && p.point.y < processor_positions[right+t].point.y && !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
            .min_by_key(|&(_, p)| abs((processor_positions[left+1].point.y+processor_positions[right+1].point.y)/2-p.point.y) + p.point.x)
            .or_else(|| {
                sorter_positions
                    .iter().enumerate()
                    .filter(|&(_, p)| processor_positions[left+t].point.y <= p.point.y && p.point.y < processor_positions[right+t].point.y && !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
                    .min_by_key(|&(_, p)| abs((processor_positions[left+1].point.y+processor_positions[right+1].point.y)/2-p.point.y) + p.point.x)
            })
            .or_else(|| {
                sorter_positions
                    .iter().enumerate()
                    .filter(|&(_, p)| processor_positions[left+t].point.y <= p.point.y && p.point.y <= processor_positions[right+t].point.y && !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
                    .min_by_key(|&(_, p)| abs((processor_positions[left+1].point.y+processor_positions[right+1].point.y)/2-p.point.y) + p.point.x)
            })
            .or_else(|| {
                sorter_positions
                    .iter().enumerate()
                    .filter(|&(_, p)| p.point.y <= processor_positions[right+t].point.y && !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
                    .min_by_key(|&(_, p)| abs((processor_positions[left+1].point.y+processor_positions[right+1].point.y)/2-p.point.y) + p.point.x)
            });

        match r {
            Some((i, &p)) => {
                used_sorter_positions.push(sorter_positions.remove(i));
                for &from in froms {
                    belts.push((from, p.point));
                }
                Some(p)
            },
            None => None
        }
    }

    processor_positions.append(&mut vec![(0, 0), (1e4 as i64, 1e4 as i64)]);
    
    let processor_positions = processor_positions
        .iter().enumerate()
        .map(|(i, &v)| Place::new(i, Point::new(v)))
        .sorted_by_key(|p| p.point.y)
        .collect::<Vec<Place>>();
    
    let sorter_positions = sorter_positions
        .iter().enumerate()
        .map(|(i, &v)| Place::new(n+i, Point::new(v)))
        .collect::<Vec<Place>>();

    let mut t = 0usize;
    if processor_positions[0].point.y < 1e4 as i64 - processor_positions.last().unwrap().point.y {
        t = 1;
    }

    let r1 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 0, find_any_position_distance, find_sorter_position, find_largest_prob_difference);
    let r2 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 1, find_any_position_distance, find_sorter_position, find_largest_prob_difference);
    let r3 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 0, find_any_position_x, find_sorter_position, find_largest_prob_difference);
    let r4 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 1, find_any_position_x, find_sorter_position, find_largest_prob_difference);
    let r5 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 0, find_any_position_center_x, find_sorter_position, find_largest_prob_difference);
    let r6 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 1, find_any_position_center_x, find_sorter_position, find_largest_prob_difference);
    let r7 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 0, find_any_position_center_xx, find_sorter_position, find_largest_prob_difference);
    let r8 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 1, find_any_position_center_xx, find_sorter_position, find_largest_prob_difference);
    let r9 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 0, find_any_position_distance, find_sorter_position1, find_largest_prob_difference);
    let r10 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 1, find_any_position_distance, find_sorter_position1, find_largest_prob_difference);
    let r11 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 0, find_any_position_x, find_sorter_position1, find_largest_prob_difference);
    let r12 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 1, find_any_position_x, find_sorter_position1, find_largest_prob_difference);
    let r13 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 0, find_any_position_center_x, find_sorter_position1, find_largest_prob_difference);
    let r14 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 1, find_any_position_center_x, find_sorter_position1, find_largest_prob_difference);
    let r15 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 0, find_any_position_center_xx, find_sorter_position1, find_largest_prob_difference);
    let r16 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 1, find_any_position_center_xx, find_sorter_position1, find_largest_prob_difference);

    let rr1 = get_better(
        get_better(
            get_better(get_better(r1, r2), get_better(r3, r4)), 
            get_better(get_better(r5, r6), get_better(r7, r8))
        ),
        get_better(
            get_better(get_better(r9, r10), get_better(r11, r12)), 
            get_better(get_better(r13, r14), get_better(r15, r16))
        )
    );

    let r1 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 0, find_any_position_distance, find_sorter_position, find_largest_prob_difference1);
    let r2 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 1, find_any_position_distance, find_sorter_position, find_largest_prob_difference1);
    let r3 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 0, find_any_position_x, find_sorter_position, find_largest_prob_difference1);
    let r4 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 1, find_any_position_x, find_sorter_position, find_largest_prob_difference1);
    let r5 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 0, find_any_position_center_x, find_sorter_position, find_largest_prob_difference1);
    let r6 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 1, find_any_position_center_x, find_sorter_position, find_largest_prob_difference1);
    let r7 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 0, find_any_position_center_xx, find_sorter_position, find_largest_prob_difference1);
    let r8 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 1, find_any_position_center_xx, find_sorter_position, find_largest_prob_difference1);
    let r9 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 0, find_any_position_distance, find_sorter_position1, find_largest_prob_difference1);
    let r10 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 1, find_any_position_distance, find_sorter_position1, find_largest_prob_difference1);
    let r11 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 0, find_any_position_x, find_sorter_position1, find_largest_prob_difference1);
    let r12 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 1, find_any_position_x, find_sorter_position1, find_largest_prob_difference1);
    let r13 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 0, find_any_position_center_x, find_sorter_position1, find_largest_prob_difference1);
    let r14 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 1, find_any_position_center_x, find_sorter_position1, find_largest_prob_difference1);
    let r15 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 0, find_any_position_center_xx, find_sorter_position1, find_largest_prob_difference1);
    let r16 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), t, 1, find_any_position_center_xx, find_sorter_position1, find_largest_prob_difference1);

    let rr2 = get_better(
        get_better(
            get_better(get_better(r1, r2), get_better(r3, r4)), 
            get_better(get_better(r5, r6), get_better(r7, r8))
        ),
        get_better(
            get_better(get_better(r9, r10), get_better(r11, r12)), 
            get_better(get_better(r13, r14), get_better(r15, r16))
        )
    );

    get_better(rr1, rr2)
}

fn t2_solve(n: usize, m: usize, k: usize, mut processor_positions: Vec<(i64, i64)>, sorter_positions: Vec<(i64, i64)>, prob: Vec<Vec<f64>>) -> Option<(i64, String, Vec<Sorter>, usize)> {
    fn solve(n: usize, m: usize, _k: usize, mut sorter_positions: Vec<Vec<Place>>, processor_positions: Vec<Place>, prob: Vec<Vec<f64>>, pt: u8, one: i64,
                find_any_position: fn(usize, usize, &Vec<Point>, &Vec<usize>, &Vec<Sorter>, &Vec<Place>, &Vec<Place>, &Vec<Place>, &mut Vec<(Point, Point)>, i64) -> Option<(u8, Place)>,
                find_sorter_position: fn(usize, usize, &Vec<Point>, &mut Vec<Vec<Place>>, &Vec<Place>, &mut Vec<Place>, &mut Vec<(Point, Point)>, i64) -> Option<Place>,
                find_largest_prob_difference: fn(&Vec<usize>, &Vec<Vec<f64>>) -> (usize, Vec<usize>, Vec<usize>, i32, u8)
            ) -> Option<(i64, String, Vec<Sorter>, usize)> {
        let mut inlet_conn = 0;
        let mut belts = vec![];
        let mut used_sorter_positions = vec![];
        let mut used_processor_positions = vec![];
        let mut fins: Vec<Sorter> = vec![];
        let mut finp = vec![];
    
        let mut q = VecDeque::new();
        q.push_back(((0..n).collect::<Vec<usize>>(), 0, n-1, vec![-1isize], vec![], 0usize));
    
        while !q.is_empty() {
            let (w, aleft, aright, mut froms, mut routes, max) = q.pop_front().unwrap();
            if w.len() == 1 {
                let r = find_any_position(aleft, aright, &froms.iter().map(|&i| fins[i as usize].place.point).collect(), &routes, &fins, &processor_positions, &used_sorter_positions, &used_processor_positions, &mut belts, one);
                match r {
                    Some((ty, p)) => {
                        for from in froms {
                            if fins[from as usize].main == -1 {
                                fins[from as usize].main = p.i as isize;
                            } else {
                                fins[from as usize].sub = p.i as isize;
                            }
                            belts.push((fins[from as usize].place.point, p.point));
            
                            if ty == 0 {
                                finp.push(Processor::new(max, p));
                                used_processor_positions.push(p);
                            }
                        }
                    },

                    None => return None
                }
                
                continue;
            }
    
            let (sorter, left, right, counter, out) = find_largest_prob_difference(&w, &prob);
            let mut nfroms = vec![];
            let mut flag = true;
            for _ in 0..counter {
                let r = find_sorter_position(aleft, aright, if froms[0] != -1 {froms.iter().map(|&i| fins[i as usize].place.point).collect()} else {&vec![Point::new((0, 5000))]}, &mut sorter_positions, &processor_positions, &mut used_sorter_positions, &mut belts, one);
                match r {
                    Some(p) => {
                        fins.push(Sorter::new(sorter, p, -1, -1, out));
                            for from in froms {
                                match from {
                                    -1 => {
                                        inlet_conn = p.i;
                                        belts.push((Point::new((0, 5000)), p.point));
                                    },
                                    _ => {
                                        if fins[from as usize].main == -1 {
                                            fins[from as usize].main = p.i as isize;
                                        } else {
                                            fins[from as usize].sub = p.i as isize;
                                        }
                                        belts.push((fins[from as usize].place.point, p.point));
                                    }
                                }
                            }
    
                            froms = vec![*froms.last().unwrap()];
                            nfroms.push(p.i as isize);
                            routes.push(p.i);
                    },
        
                    None => {
                        let r = find_any_position(aleft, aright, &froms.iter().map(|&i| fins[i as usize].place.point).collect(), &routes, &fins, &processor_positions, &used_sorter_positions, &used_processor_positions, &mut belts, one);
                        match r {
                            Some((ty, p)) => {
                                for from in froms {
                                    match from {
                                        -1 => {
                                            inlet_conn = p.i;
                                            belts.push((Point::new((0, 5000)), p.point));
                                        },
                                        _ => {
                                            if fins[from as usize].main == -1 {
                                                fins[from as usize].main = p.i as isize;
                                            } else {
                                                fins[from as usize].sub = p.i as isize;
                                            }
                                            belts.push((fins[from as usize].place.point, p.point));
                                        }
                                    }
                                }
                
                                if ty == 0 {
                                    finp.push(Processor::new(max, p));
                                    used_processor_positions.push(p);
                                }
                            },
                            None => return None
                        }
                    }
                }

                if flag {
                    match pt {
                        0 => {
                            q.push_front((
                                left.clone(), aleft, aright-right.len(), froms.clone(), routes.clone(), prob[sorter]
                                    .iter().enumerate()
                                    .filter(|(i, _)| left.contains(i))
                                    .max_by(|(_, p1), (_, p2)| p1.partial_cmp(p2).unwrap()).unwrap().0
                            ));
                        },
                        _ => {
                            q.push_back((
                                    left.clone(), aleft, aright-right.len(), froms.clone(), routes.clone(), prob[sorter]
                                        .iter().enumerate()
                                        .filter(|(i, _)| left.contains(i))
                                        .max_by(|(_, p1), (_, p2)| p1.partial_cmp(p2).unwrap()).unwrap().0
                            ));
                        }
                    }
                    
                    q.push_front((
                        right.clone(), aleft+left.len(), aright, nfroms, routes.clone(), prob[sorter]
                            .iter().enumerate()
                            .filter(|(i, _)| right.contains(i))
                            .max_by(|(_, p1), (_, p2)| p1.partial_cmp(p2).unwrap()).unwrap().0
                    ));
                }
            }
        }
    
        let mut pset = BTreeSet::new();
        let mut vset = BTreeSet::new();
        for i in 0..n {
            pset.insert(i);
            vset.insert(i);
        }
    
        let mut finp = finp.iter()
            .map(|p| (p.place.i, p.i))
            .collect::<Vec<_>>();
    
        for (i, v) in &finp {
            pset.remove(i);
            vset.remove(v);
        }
    
        for (i, v) in pset.iter().zip(vset.iter()) {
            finp.push((*i, *v));
        }
    
        let ds: Vec<usize> = finp.iter()
            .sorted_by_key(|&(i, _)| i)
            .map(|&(_, v)| v)
            .collect();
        
        fins.sort_by_key(|s| s.place.i);
        let mut cs = vec![(usize::MAX, -1isize, -1isize); m];
        
        for s in &fins {
            let idx = s.place.i - n;
            cs[idx] = (s.i, s.sub, s.main);
        }
    
        let finp = ds.iter()
            .map(|&v| v.to_string())
            .join(" ");

        Some((compute_score(n, m, &prob, &ds, inlet_conn, &cs), finp, fins, inlet_conn))
    }

    fn find_any_position_distance(left: usize, right: usize, froms: &Vec<Point>, routes: &Vec<usize>, fins: &Vec<Sorter>, processor_positions: &Vec<Place>, used_sorter_positions: &Vec<Place>, used_processor_positions: &Vec<Place>, belts: &mut Vec<(Point, Point)>, _one: i64) -> Option<(u8, Place)> {
        processor_positions[left+1..=right+1].iter()
            .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
            .min_by_key(|p| abs((processor_positions[left+1].point.y+processor_positions[right+1].point.y)/2-p.point.y))
            .map(| &p| (0, p))
            .or_else(|| 
                used_sorter_positions.iter()
                    .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)) && !routes.contains(&p.i))
                    .min_by_key(|p| froms.iter().map(|&v| p.point.get_distance(&v)).sum::<usize>())
                    .map(| &p| (1, p))
            )
            .or_else(|| 
                used_processor_positions.iter()
                    .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
                    .min_by_key(|p| froms.iter().map(|&v| p.point.get_distance(&v)).sum::<usize>())
                    .map(| &p| (1, p))
            )
    }

    fn find_any_position_x(left: usize, right: usize, froms: &Vec<Point>, routes: &Vec<usize>, fins: &Vec<Sorter>, processor_positions: &Vec<Place>, used_sorter_positions: &Vec<Place>, used_processor_positions: &Vec<Place>, belts: &mut Vec<(Point, Point)>, _one: i64) -> Option<(u8, Place)> {
        processor_positions[left+1..=right+1].iter()
            .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
            .min_by_key(|p| abs((processor_positions[left+1].point.y+processor_positions[right+1].point.y) / 2 - p.point.y))
            .map(| &p| (0, p))
            .or_else(|| 
                used_sorter_positions.iter()
                    .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)) && !routes.contains(&p.i))
                    .min_by_key(|p| p.point.x)
                    .map(| &p| (1, p))
            )
            .or_else(|| 
                used_processor_positions.iter()
                    .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
                    .min_by_key(|p| p.point.x)
                    .map(| &p| (1, p))
            )
    }

    fn find_any_position_center_x(left: usize, right: usize, froms: &Vec<Point>, routes: &Vec<usize>, fins: &Vec<Sorter>, processor_positions: &Vec<Place>, used_sorter_positions: &Vec<Place>, used_processor_positions: &Vec<Place>, belts: &mut Vec<(Point, Point)>, one: i64) -> Option<(u8, Place)> {
        processor_positions[left+1..=right+1].iter()
            .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
            .min_by_key(|p| abs((processor_positions[left+1].point.y+processor_positions[right+1].point.y) / 2 - p.point.y))
            .map(| &p| (0, p))
            .or_else(|| 
                used_sorter_positions.iter()
                    .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)) && !routes.contains(&p.i))
                    .min_by_key(|p| abs((left+right) as i64 / 2 * one - p.point.y) + p.point.x)
                    .map(| &p| (1, p))
            )
            .or_else(|| 
                used_processor_positions.iter()
                    .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
                    .min_by_key(|p| abs((left+right) as i64 / 2 * one - p.point.y))
                    .map(| &p| (1, p))
            )
    }

    fn find_any_position_center_xx(left: usize, right: usize, froms: &Vec<Point>, routes: &Vec<usize>, fins: &Vec<Sorter>, processor_positions: &Vec<Place>, used_sorter_positions: &Vec<Place>, used_processor_positions: &Vec<Place>, belts: &mut Vec<(Point, Point)>, one: i64) -> Option<(u8, Place)> {
        processor_positions[left+1..=right+1].iter()
            .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
            .min_by_key(|p| abs((processor_positions[left+1].point.y+processor_positions[right+1].point.y) / 2 - p.point.y))
            .map(| &p| (0, p))
            .or_else(|| 
                used_sorter_positions.iter()
                    .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)) && !routes.contains(&p.i))
                    .min_by_key(|p| abs((left+right) as i64 / 2 * one - p.point.y) + froms.iter().map(|&v| abs(p.point.x - v.x)).sum::<usize>())
                    .map(| &p| (1, p))
            )
            .or_else(|| 
                used_processor_positions.iter()
                    .filter(|p| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
                    .min_by_key(|p| abs((left+right) as i64 / 2 * one - p.point.y) + froms.iter().map(|&v| abs(p.point.x - v.x)).sum::<usize>())
                    .map(| &p| (1, p))
        )
    }
    
    fn find_sorter_position(left: usize, right: usize, froms: &Vec<Point>, sorter_positions: &mut Vec<Vec<Place>>, _processor_positions: &Vec<Place>, used_sorter_positions: &mut Vec<Place>, belts: &mut Vec<(Point, Point)>, _one: i64) -> Option<Place> {
        let r = sorter_positions[left..=right]
            .iter().enumerate()
            .flat_map(|(i, v)| v.iter().enumerate().map(move |(j, w)| (left+i, j, *w)))
            .filter(|&(_, _, p)| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
            .min_by_key(|&(_, _, p)| p.point.x)
            .or_else(|| {
                sorter_positions[..=right]
                    .iter().enumerate()
                    .flat_map(|(i, v)| v.iter().enumerate().map(move |(j, w)| (i, j, *w)))
                    .filter(|&(_, _, p)| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
                    .min_by_key(|&(_, _, p)| p.point.x)
            });

        match r {
            Some((i, j, p)) => {
                used_sorter_positions.push(sorter_positions[i].remove(j));
                for &from in froms {
                    belts.push((from, p.point));
                }
                Some(p)
            },
            None => None
        }
    }

    fn find_sorter_position1(left: usize, right: usize, froms: &Vec<Point>, sorter_positions: &mut Vec<Vec<Place>>, _processor_positions: &Vec<Place>, used_sorter_positions: &mut Vec<Place>, belts: &mut Vec<(Point, Point)>, one: i64) -> Option<Place> {
        let r = sorter_positions[left..=right]
            .iter().enumerate()
            .flat_map(|(i, v)| v.iter().enumerate().map(move |(j, w)| (left+i, j, *w)))
            .filter(|&(_, _, p)| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
            .min_by_key(|&(_, _, p)| abs((left+right) as i64 / 2 * one - p.point.y) + p.point.x)
            .or_else(|| {
                sorter_positions[..=right]
                    .iter().enumerate()
                    .flat_map(|(i, v)| v.iter().enumerate().map(move |(j, w)| (i, j, *w)))
                    .filter(|&(_, _, p)| !froms.iter().any(|&v| p.point.check_intersection(&v, &belts)))
                    .min_by_key(|&(_, _, p)| abs((left+right) as i64 / 2 * one - p.point.y) + p.point.x)
            });

        match r {
            Some((i, j, p)) => {
                used_sorter_positions.push(sorter_positions[i].remove(j));
                for &from in froms {
                    belts.push((from, p.point));
                }
                Some(p)
            },
            None => None
        }
    }

    processor_positions.append(&mut vec![(0, 0), (1e4 as i64, 1e4 as i64)]);
    
    let processor_positions = processor_positions
        .iter().enumerate()
        .map(|(i, &v)| Place::new(i, Point::new(v)))
        .sorted_by_key(|p| p.point.y)
        .collect::<Vec<Place>>();
    
    let sorter_positionsb = sorter_positions
        .iter().enumerate()
        .map(|(i, &v)| Place::new(n+i, Point::new(v)))
        .collect::<Vec<Place>>();

    let mut sorter_positions = vec![vec![];n];
    let one = 1e4 as i64 / n as i64;
    for i in sorter_positionsb {
        let mut index = i.point.y as i64 / one;
        if index == n as i64 {
            index -= 1;
        }
        sorter_positions[index as usize].push(i);
    }

    let r1 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 0, one, find_any_position_distance, find_sorter_position, find_largest_prob_difference);
    let r2 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 1, one, find_any_position_distance, find_sorter_position, find_largest_prob_difference);
    let r3 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 0, one, find_any_position_x, find_sorter_position, find_largest_prob_difference);
    let r4 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 1, one, find_any_position_x, find_sorter_position, find_largest_prob_difference);
    let r5 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 0, one, find_any_position_center_x, find_sorter_position, find_largest_prob_difference);
    let r6 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 1, one, find_any_position_center_x, find_sorter_position, find_largest_prob_difference);
    let r7 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 0, one, find_any_position_center_xx, find_sorter_position, find_largest_prob_difference);
    let r8 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 1, one, find_any_position_center_xx, find_sorter_position, find_largest_prob_difference);
    let r9 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 0, one, find_any_position_distance, find_sorter_position1, find_largest_prob_difference);
    let r10 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 1, one, find_any_position_distance, find_sorter_position1, find_largest_prob_difference);
    let r11 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 0, one, find_any_position_x, find_sorter_position1, find_largest_prob_difference);
    let r12 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 1, one, find_any_position_x, find_sorter_position1, find_largest_prob_difference);
    let r13 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 0, one, find_any_position_center_x, find_sorter_position1, find_largest_prob_difference);
    let r14 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 1, one, find_any_position_center_x, find_sorter_position1, find_largest_prob_difference);
    let r15 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 0, one, find_any_position_center_xx, find_sorter_position1, find_largest_prob_difference);
    let r16 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 1, one, find_any_position_center_xx, find_sorter_position1, find_largest_prob_difference);

    let rr1 = get_better(
        get_better(
            get_better(get_better(r1, r2), get_better(r3, r4)), 
            get_better(get_better(r5, r6), get_better(r7, r8))
        ),
        get_better(
            get_better(get_better(r9, r10), get_better(r11, r12)), 
            get_better(get_better(r13, r14), get_better(r15, r16))
        ),
    );

    let r1 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 0, one, find_any_position_distance, find_sorter_position, find_largest_prob_difference1);
    let r2 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 1, one, find_any_position_distance, find_sorter_position, find_largest_prob_difference1);
    let r3 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 0, one, find_any_position_x, find_sorter_position, find_largest_prob_difference1);
    let r4 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 1, one, find_any_position_x, find_sorter_position, find_largest_prob_difference1);
    let r5 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 0, one, find_any_position_center_x, find_sorter_position, find_largest_prob_difference1);
    let r6 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 1, one, find_any_position_center_x, find_sorter_position, find_largest_prob_difference1);
    let r7 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 0, one, find_any_position_center_xx, find_sorter_position, find_largest_prob_difference1);
    let r8 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 1, one, find_any_position_center_xx, find_sorter_position, find_largest_prob_difference1);
    let r9 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 0, one, find_any_position_distance, find_sorter_position1, find_largest_prob_difference1);
    let r10 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 1, one, find_any_position_distance, find_sorter_position1, find_largest_prob_difference1);
    let r11 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 0, one, find_any_position_x, find_sorter_position1, find_largest_prob_difference1);
    let r12 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 1, one, find_any_position_x, find_sorter_position1, find_largest_prob_difference1);
    let r13 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 0, one, find_any_position_center_x, find_sorter_position1, find_largest_prob_difference1);
    let r14 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 1, one, find_any_position_center_x, find_sorter_position1, find_largest_prob_difference1);
    let r15 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 0, one, find_any_position_center_xx, find_sorter_position1, find_largest_prob_difference1);
    let r16 = solve(n, m, k, sorter_positions.clone(), processor_positions.clone(), prob.clone(), 1, one, find_any_position_center_xx, find_sorter_position1, find_largest_prob_difference1);

    let rr2 = get_better(
        get_better(
            get_better(get_better(r1, r2), get_better(r3, r4)), 
            get_better(get_better(r5, r6), get_better(r7, r8))
        ),
        get_better(
            get_better(get_better(r9, r10), get_better(r11, r12)), 
            get_better(get_better(r13, r14), get_better(r15, r16))
        ),
    );

    get_better(rr1, rr2)
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        mut processor_positions: [(i64, i64); n],
        sorter_positions: [(i64, i64); m],
        prob: [[f64; n]; k],
    }

    let r = get_better(t1_solve(n, m, k, processor_positions.clone(), sorter_positions.clone(), prob.clone()), 
                                                        t2_solve(n, m, k, processor_positions.clone(), sorter_positions.clone(), prob.clone()));

    if let Some((_, finp, fins, inlet_conn)) = r {
        println!("{}", finp);
        println!("{}", inlet_conn);
        let mut now = 0;
        for s in fins {
            while now < s.place.i-n {
                println!("-1");
                now += 1;
            }
            println!("{} {} {}", s.i, s.sub, s.main);
            now += 1;
        }

        while now < m {
            println!("-1");
            now += 1;
        }
    } else {
        let proc_assign = (0..n).map(|i| i.to_string()).join(" ");
        let inlet = (0, 5000);
        let nearest_i = sorter_positions
        .iter()
        .enumerate()
        .min_by_key(|(_, &(x, y))| {
            let dx = x - inlet.0;
            let dy = y - inlet.1;
            (dx as i64) * (dx as i64) + (dy as i64) * (dy as i64)
        })
        .unwrap()
        .0;
    
        let inlet_conn = n + nearest_i;
        let first_row = &prob[0];
        let imax = first_row
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap()
            .0;
        let imin = first_row
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap()
            .0;
        
        let sorter_assigns = (0..m)
            .map(|i| {
                if i == nearest_i {
                    format!("0 {} {}", imax, imin)
                } else {
                    "-1".to_string()
                }
            })
            .collect::<Vec<_>>();
        println!("{}", proc_assign);
        println!("{}", inlet_conn);
        for assign in sorter_assigns {
            println!("{}", assign);
        }
    }
}