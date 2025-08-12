use std::collections::{BTreeSet, VecDeque};

use indexmap::IndexMap;
use itertools::Itertools;
use num::abs;
#[allow(unused_imports, non_snake_case)]
use proconio::{fastout, input, marker::{Bytes, Chars, Usize1}};

#[derive(Debug, Clone, Copy)]
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
    sub: isize
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
    fn new(i: usize, place: Place, main: isize, sub: isize) -> Self {
        Self { i, place, main, sub }
    }
}

impl Processor {
    fn new(i: usize, place: Place) -> Self {
        Self { i, place }
    }
}

fn find_largest_prob_difference(w: &Vec<usize>, prob: &Vec<Vec<f64>>) -> (usize, Vec<usize>, Vec<usize>) {
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
        (sorter, prob[sorter][waste..].iter().map(|&(i, _)| i).collect(), prob[sorter][..waste].iter().map(|&(i, _)| i).collect())
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
    
        (max.0, prob[max.0][max.1..].iter().map(|&(i, _)| i).collect(), prob[max.0][..max.1].iter().map(|&(i, _)| i).collect())
        
    }
}

fn find_sorter_position(t: usize, left: usize, right: usize, from: Point, sorter_positions: &mut Vec<Place>, processor_positions: &Vec<Place>, used_sorter_positions: &mut Vec<Place>, belts: &mut Vec<(Point, Point)>) -> Option<Place> {
    let r = sorter_positions
        .iter().enumerate()
        .filter(|&(_, p)| processor_positions[left+t].point.y < p.point.y && p.point.y < processor_positions[right+t].point.y && !from.check_intersection(&p.point, belts))
        .min_by_key(|&(_, p)| p.point.x)
        .or_else(|| {
            sorter_positions
                .iter().enumerate()
                .filter(|&(_, p)| processor_positions[left+t].point.y <= p.point.y && p.point.y < processor_positions[right+t].point.y && !from.check_intersection(&p.point, belts))
                .min_by_key(|&(_, p)| p.point.x)
        })
        .or_else(|| {
            sorter_positions
                .iter().enumerate()
                .filter(|&(_, p)| processor_positions[left+t].point.y <= p.point.y && p.point.y <= processor_positions[right+t].point.y && !from.check_intersection(&p.point, belts))
                .min_by_key(|&(_, p)| p.point.x)
        })
        .or_else(|| {
            sorter_positions
                .iter().enumerate()
                .filter(|&(_, p)| p.point.y <= processor_positions[right+t].point.y && !from.check_intersection(&p.point, belts))
                .min_by_key(|&(_, p)| p.point.x)
        });

    match r {
        Some((i, &p)) => {
            used_sorter_positions.push(sorter_positions.remove(i));
            belts.push((from, p.point));
            Some(p)
        },
        None => None
    }
}

fn find_any_position(left: usize, right: usize, from: usize, fins: &Vec<Sorter>, processor_positions: &Vec<Place>, used_sorter_positions: &Vec<Place>, used_processor_positions: &Vec<Place>, belts: &Vec<(Point, Point)>) -> (u8, Place) {
    //dbg!(left, right, from, &processor_positions[left+1..=right+1]);
    processor_positions[left+1..=right+1].iter()
        .filter(|p| !fins[from].place.point.check_intersection(&p.point, &belts))
        .min_by_key(|p| fins[from].place.point.get_distance(&p.point))
        .map(| &p| (0, p))
        .or_else(|| 
            used_processor_positions.iter()
                .filter(|p| !fins[from].place.point.check_intersection(&p.point, &belts))
                .min_by_key(|p| abs((processor_positions[left+1].point.y+processor_positions[right+1].point.y)/2-p.point.y))
                .map(| &p| (1, p))
        ).or_else(|| 
            used_sorter_positions.iter()
                .filter(|p| !fins[from].place.point.check_intersection(&p.point, &belts))
                .max_by_key(|p| p.point.x)
                .map(| &p| (1, p))
        ).unwrap()
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

    processor_positions.append(&mut vec![(0, 0), (1e4 as i64, 1e4 as i64)]);
    
    let processor_positions = processor_positions
        .iter().enumerate()
        .map(|(i, &v)| Place::new(i, Point::new(v)))
        .sorted_by_key(|p| p.point.y)
        .collect::<Vec<Place>>();
    
    let mut sorter_positions = sorter_positions
        .iter().enumerate()
        .map(|(i, &v)| Place::new(n+i, Point::new(v)))
        .collect::<Vec<Place>>();

    let mut t = 0usize;
    if processor_positions[0].point.y < 1e4 as i64 - processor_positions.last().unwrap().point.y {
        t = 1;
    }

    let mut inlet_conn = 0;
    let mut belts = vec![];
    let mut used_sorter_positions = vec![];
    let mut used_processor_positions = vec![];
    let mut fins: Vec<Sorter> = vec![];
    let mut finp = vec![];

    let mut q = VecDeque::new();
    q.push_back(((0..n).collect::<Vec<usize>>(), 0, n-1, -1isize, 0usize));

    while !q.is_empty() {
        let (w, aleft, aright, from, max) = q.pop_front().unwrap();
        if w.len() == 1 {
            let (ty, p) = find_any_position(aleft, aright, from as usize, &fins, &processor_positions, &used_sorter_positions, &used_processor_positions, &belts);
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
            
            continue;
        }

        let (sorter, left, right) = find_largest_prob_difference(&w, &prob);
        let r = find_sorter_position(t, aleft, aright, if from != -1 {fins[from as usize].place.point} else {Point::new((0, 5000))}, &mut sorter_positions, &processor_positions, &mut used_sorter_positions, &mut belts);
        match r {
            Some(p) => {
                fins.push(Sorter::new(sorter, p, -1, -1));
                match from {
                    -1 => inlet_conn = p.i,
                    _ => {
                        if fins[from as usize].main == -1 {
                            fins[from as usize].main = p.i as isize;
                        } else {
                            fins[from as usize].sub = p.i as isize;
                        }
                    }
                }
                q.push_back((
                    left.clone(), aleft, aright-right.len(), (fins.len()-1) as isize, prob[sorter]
                        .iter().enumerate()
                        .filter(|(i, _)| left.contains(i))
                        .max_by(|(_, p1), (_, p2)| p1.partial_cmp(p2).unwrap()).unwrap().0
                ));
                q.push_front((
                    right.clone(), aleft+left.len(), aright, (fins.len()-1) as isize, prob[sorter]
                        .iter().enumerate()
                        .filter(|(i, _)| right.contains(i))
                        .max_by(|(_, p1), (_, p2)| p1.partial_cmp(p2).unwrap()).unwrap().0
                ));
            },

            None => {
                let (ty, p) = find_any_position(aleft, aright, from as usize, &fins, &processor_positions, &used_sorter_positions, &used_processor_positions, &belts);
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
    
    let finp = finp.iter()
        .sorted_by_key(|&(i, _)| i)
        .map(|(_, i)| i)
        .join(" ");
    fins.sort_by_key(|s| s.place.i);
    
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
}